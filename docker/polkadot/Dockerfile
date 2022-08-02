FROM debian:bookworm-slim
RUN apt-get update -y && apt-get install -y ca-certificates
ADD https://github.com/paritytech/polkadot/releases/download/v0.9.26/polkadot /usr/bin/
RUN chmod +x /usr/bin/polkadot
CMD ["/usr/bin/polkadot"]
