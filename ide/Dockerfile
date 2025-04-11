FROM rust:slim@sha256:3f391b0678a6e0c88fd26f13e399c9c515ac47354e3cadfee7daee3b21651a4f

WORKDIR /setup


#ADD ca/ssl.crt /usr/local/share/ca-certificates/ssl.crt
RUN update-ca-certificates

# Set up dependencies
SHELL [ "/bin/bash", "-c" ]
RUN apt update -y && apt upgrade -y && apt install -y pkg-config build-essential curl wget

# Add Golang
RUN wget https://go.dev/dl/go1.24.2.linux-amd64.tar.gz
RUN rm -rf /usr/local/go && tar -C /usr/local -xzf go1.24.2.linux-amd64.tar.gz
RUN rm go1.24.2.linux-amd64.tar.gz
ENV PATH=$PATH:/usr/local/go/bin
RUN go version

RUN cargo install cw-warp

WORKDIR /root/projects


RUN rm -rf /root/.cargo/registry
CMD [ "tail", "-f", "/dev/null" ]