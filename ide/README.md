# Integrated Warp Development Environment

The dockerized Warp environment will allow you to access the full Warp environment in the most portable, dependency-free way possible. This can be used for:

* Local development on VMs, unconfigured workstations, temporary machines
* CI/CD pipelines and cloud automation scenarios

# Requirements

The only requirement to set up Warp environment is Docker. No need for local installations of Rust, Golang, specific blockchain node binaries, npm, or any other packages.

# Local Development

To use this environment for local development, make sure your machine has the following things installed:

* a Unix shell like `bash`
* Docker
* VS Code
* Dev Containers extension for VS Code

## Initialize the environment image

Run the following command to start a Warp docker container:

```sh
docker run --name warp-env -d \
    -v $HOME/warp-env:/root/projects \
    -v /var/run/docker.sock:/var/run/docker.sock \
    ghcr.io/cw-warp/ide
```