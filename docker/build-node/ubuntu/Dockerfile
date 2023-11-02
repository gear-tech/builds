#
# Build:
#
#     docker build .
#

FROM --platform=linux/amd64 ubuntu:18.04

WORKDIR /root

# Install deps
RUN apt update -y && apt upgrade -y
RUN apt install -y build-essential clang cmake git unzip wget

# Install protoc v3
RUN wget https://github.com/protocolbuffers/protobuf/releases/download/v3.20.2/protoc-3.20.2-linux-x86_64.zip
RUN unzip protoc-3.20.2-linux-x86_64.zip -d /usr/local
RUN rm protoc-3.20.2-linux-x86_64.zip
RUN protoc --version

# Install Rust and toolchains
RUN  wget https://sh.rustup.rs/rustup-init.sh
RUN chmod +x rustup-init.sh
RUN ./rustup-init.sh -y
ENV PATH="/root/.cargo/bin:$PATH"
RUN rustup default stable
RUN cargo --version
RUN rustc --version
RUN rustup update nightly && rustup target add wasm32-unknown-unknown --toolchain nightly

# Clone & build
RUN git clone https://github.com/gear-tech/gear.git
WORKDIR /root/gear
ENV CARGO_UNSTABLE_SPARSE_REGISTRY=true
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
RUN cargo fetch
RUN cargo build -rp gear-cli
