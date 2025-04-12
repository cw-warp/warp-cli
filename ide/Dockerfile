FROM golang:1.24 AS go_builder

RUN apt update -y && apt install -y unzip

# Injective
RUN git clone https://github.com/InjectiveLabs/injective-chain-releases.git injective-releases
RUN cd injective-releases && git describe --tags `git rev-list --tags --max-count=1` > ../latest.txt
RUN wget https://github.com/InjectiveLabs/injective-chain-releases/releases/download/$(cat latest.txt)/linux-amd64.zip
RUN unzip linux-amd64.zip
RUN ls -al
RUN chmod +x injectived peggo

# SEI
RUN git clone https://github.com/sei-protocol/sei-chain sei
RUN cd sei && make build
RUN pwd && ls sei/build

FROM rust:slim@sha256:3f391b0678a6e0c88fd26f13e399c9c515ac47354e3cadfee7daee3b21651a4f
COPY --from=go_builder /go/libwasmvm.x86_64.so /usr/lib/libwasmvm.x86_64.so
COPY --from=go_builder /go/injectived /usr/local/bin/injectived
COPY --from=go_builder /go/peggo /usr/local/bin/peggo
COPY --from=go_builder /go/sei/build/seid /usr/local/bin/seid

WORKDIR /setup


#ADD ca/ssl.crt /usr/local/share/ca-certificates/ssl.crt
RUN update-ca-certificates

# Set up dependencies
SHELL [ "/bin/bash", "-c" ]
RUN apt update -y && apt upgrade -y && apt install -y pkg-config build-essential curl wget git libssl-dev

# Add Golang
# RUN wget https://go.dev/dl/go1.24.2.linux-amd64.tar.gz
# RUN rm -rf /usr/local/go && tar -C /usr/local -xzf go1.24.2.linux-amd64.tar.gz
# RUN rm go1.24.2.linux-amd64.tar.gz
# ENV PATH=$PATH:/usr/local/go/bin
# RUN go version

# Archwayd
RUN wget https://github.com/archway-network/archway/releases/download/v9.0.0/archwayd_linux_amd64
RUN chmod +x archwayd_linux_amd64
RUN mv archwayd_linux_amd64 /usr/local/bin/archwayd

RUN cargo install cw-warp



WORKDIR /root/projects


RUN rm -rf /root/.cargo/registry
CMD [ "tail", "-f", "/dev/null" ]