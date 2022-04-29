ssmpuller
================

Generates a systemd EnvironmentFile from [AWS Systems Manager parameters](https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-parameter-store.html).

## Installation

To install via [Cargo](https://doc.rust-lang.org/cargo/):

    cargo install ssmpuller

To install from source, first, [install Rust](https:://rustup.rs/), and then run:

    git clone github.com/l1na-forever/ssmpuller
    cd ssmpuller
    cargo install

**Binary releases can be found on the [Releases page](https://github.com/l1na-forever/ssmpuller/releases/).**

## Usage

To get help, run the command:

    ssmpuller --help

To generate a [systemd EnvironmentFile](https://www.freedesktop.org/software/systemd/man/systemd.exec.html#EnvironmentFile=) from secure parameters stored in SSM:

    ssmpuller /etc/myservice/myservice.conf MY_API_KEY

This will generate a file that can be used to set environment variables via the `EnvironmentFile=` directive. An example file might look like:

    MY_API_KEY='secret_token_goes_here'

To make use of the generated file as part of a [systemd service unit](https://www.freedesktop.org/software/systemd/man/systemd.service.html), set the `EnvironmentFile=` directive:

    [Unit]
    Description=Example systemd service

    [Service]
    ExecStart=/usr/bin/myservice
    EnvironmentFile=/etc/myservice/myservice.conf

`ssmpuller`'s intended usage is to bootstrap an instance's long-lived, static credentials (e.g., 3rd-party API keys) at provision time, using the IAM instance role. Wherever possible, make use of a temporary/federated credentials and the instance role credentials to make service calls. Do **not** use `ssmpuller` to manage any AWS credentials; [**use IAM instance roles instead**](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/iam-roles-for-amazon-ec2.html).

## Status

`ssmpuller` is considered feature-complete, and is offered as-is. Bug reports and pull requests are most welcome!

## Licence

Copyright © 2022 Lina

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
