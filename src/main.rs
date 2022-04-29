use aws_sdk_ssm::{error::GetParametersError, model::Parameter, types::SdkError as SsmSdkError};
use aws_types::sdk_config::SdkConfig;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use thiserror::Error;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "ssmpuller",
    about = "Generates a systemd EnvironmentFile from AWS Systems Manager parameters."
)]
struct PullerOptions {
    /// Output path for the generated EnvironmentFile
    #[structopt(parse(from_os_str))]
    path: PathBuf,

    /// Parameter names, whose decrypted values will be written to the EnvironmentFile
    parameters: Vec<String>,
}

#[derive(Error, Debug)]
pub enum PullerError {
    #[error("error calling service dependency")]
    Dependency(String),

    #[error("invalid parameter `{0}`")]
    InvalidParameter(String),

    #[error("error writing to disk")]
    IO(#[from] std::io::Error),
}

impl From<SsmSdkError<GetParametersError>> for PullerError {
    fn from(e: SsmSdkError<GetParametersError>) -> Self {
        Self::Dependency(format!("Error calling SSM: {:?}", e))
    }
}

#[derive(Debug)]
pub struct Puller {
    client: aws_sdk_ssm::Client,
}

impl Puller {
    pub fn new(config: &SdkConfig) -> Self {
        Self {
            client: aws_sdk_ssm::Client::new(config),
        }
    }

    pub async fn get_parameters(
        &mut self,
        parameters: Vec<String>,
    ) -> Result<Vec<Parameter>, PullerError> {
        // Get response from SSM with decrypted values
        let response = self
            .client
            .get_parameters()
            .set_names(Some(parameters))
            .with_decryption(true)
            .send()
            .await?;

        // Validate that no parameters queried were invalid
        if let Some(invalid_parameters) = response.invalid_parameters {
            if !invalid_parameters.is_empty() {
                return Err(PullerError::InvalidParameter(invalid_parameters[0].clone()));
            }
        }

        Ok(response.parameters.unwrap())
    }
}

pub fn write_environment_file(parameters: &[Parameter], path: &PathBuf) -> Result<(), PullerError> {
    // Convert the response into the systemd EnvironmentFile format
    // See: https://www.freedesktop.org/software/systemd/man/systemd.exec.html#EnvironmentFile=
    let lines: String = parameters
        .iter()
        .map(|p| {
            format!(
                "{}='{}'\n",
                p.name.as_ref().unwrap(),
                p.value.as_ref().unwrap()
            )
        })
        .collect();

    // Write the output to disk
    fs::write(path, &lines)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), PullerError> {
    let PullerOptions { path, parameters } = PullerOptions::from_args();
    let config = aws_config::load_from_env().await;

    let mut puller = Puller::new(&config);
    let parameters = puller.get_parameters(parameters).await?;

    write_environment_file(&parameters, &path)
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use aws_sdk_ssm::model::Parameter;
    use tempfile::NamedTempFile;

    use crate::write_environment_file;

    #[test]
    fn test_write_environment_file() {
        let parameters = vec![
            make_parameter("S1_TOKEN", "S1_CRED"),
            make_parameter("S2_TOKEN", "S2_CRED"),
        ];
        let mut file = NamedTempFile::new().expect("unable to create tempfile");
        write_environment_file(&parameters, &file.path().into())
            .expect("expected envfile write to succeed");

        // Read file and ensure it looks right
        let mut result = String::new();
        file.read_to_string(&mut result).unwrap();

        assert_eq!(
            result,
            r#"S1_TOKEN='S1_CRED'
S2_TOKEN='S2_CRED'
"#
        );
    }

    fn make_parameter(name: &str, value: &str) -> Parameter {
        Parameter::builder().name(name).value(value).build()
    }
}
