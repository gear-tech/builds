#
# Build on macOS:
#
#     docker build --platform linux/amd64 .
#

FROM amazonlinux:latest

WORKDIR /root

# Install deps
RUN yum update -y && yum install -y amazon-linux-extras clang gcc git gzip make tar wget
RUN clang --version
RUN gcc --version
RUN c++ --version
RUN git --version

# Install CMake v3
RUN wget https://cmake.org/files/v3.24/cmake-3.24.0-linux-x86_64.sh
RUN chmod +x cmake-3.24.0-linux-x86_64.sh
RUN ./cmake-3.24.0-linux-x86_64.sh --skip-license --prefix=/usr/local
RUN rm cmake-3.24.0-linux-x86_64.sh
RUN cmake --version

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
RUN cargo build -rp gear-cli
