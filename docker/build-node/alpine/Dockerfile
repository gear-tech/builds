#
# Build on macOS:
#
#     docker build .
#

FROM --platform=linux/amd64 alpine:3

WORKDIR /root

# Install deps
RUN apk update
RUN apk add git

# RUN yum update -y && yum install -y clang gcc git gzip make tar wget
# RUN clang --version
# RUN gcc --version
# RUN c++ --version
# RUN git --version

# # Install CMake v3
# RUN wget https://cmake.org/files/v3.24/cmake-3.24.0-linux-x86_64.sh
# RUN chmod +x cmake-3.24.0-linux-x86_64.sh
# RUN ./cmake-3.24.0-linux-x86_64.sh --skip-license --prefix=/usr/local
# RUN rm cmake-3.24.0-linux-x86_64.sh
# RUN cmake --version

# # Install Rust and toolchains
# RUN  wget https://sh.rustup.rs/rustup-init.sh
# RUN chmod +x rustup-init.sh
# RUN ./rustup-init.sh -y
# ENV PATH="/root/.cargo/bin:$PATH"
# RUN rustup default stable
# RUN cargo --version
# RUN rustc --version
# RUN rustup update nightly && rustup target add wasm32-unknown-unknown --toolchain nightly

# Clone & build
RUN git clone https://github.com/gear-tech/gear.git
WORKDIR /root/gear
ENV CARGO_UNSTABLE_SPARSE_REGISTRY=true
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
RUN apk add rust
RUN apk add cargo
RUN cargo fetch
RUN apk add protoc
RUN apk add g++
RUN apk add rustup
RUN  wget https://sh.rustup.rs/rustup-init.sh
RUN chmod +x rustup-init.sh
RUN ./rustup-init.sh -y
ENV PATH="/root/.cargo/bin:$PATH"
RUN rustup target add x86_64-unknown-linux-musl
RUN apk add jemalloc
RUN apk add clang
RUN apk add make
RUN cargo b -rp tikv-jemalloc-sys --target x86_64-unknown-linux-musl
RUN apk add binutils
RUN apk add llvm-dev
RUN apk add clang-dev
RUN apk add clang-static
RUN apk add gcompat
# RUN find / -name "libclang*" 2> /dev/null && false
ENV RUSTFLAGS=-Ctarget-feature=-crt-static
RUN mkdir lib
RUN cp /usr/lib/llvm16/lib/libclang.a ./lib/libclang.so
ENV LIBCLANG_PATH=/root/lib
RUN cargo b -rp librocksdb-sys --target x86_64-unknown-linux-musl
RUN cargo build -rp gear-cli --target x86_64-unknown-linux-musl
