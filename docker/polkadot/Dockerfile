FROM debian:bookworm-slim
RUN apt-get update -y && apt-get install -y ca-certificates
ADD https://github.com/paritytech/polkadot/releases/download/v0.9.26/polkadot /usr/bin/
RUN chmod +x /usr/bin/polkadot
RUN useradd -m -u 1000 -U -s /bin/sh -d /polkadot polkadot && \
	mkdir -p /polkadot/.local/share && \
	mkdir /data && \
	chown -R polkadot:polkadot /data && \
	ln -s /data /polkadot/.local/share/polkadot

USER root
CMD ["/usr/bin/polkadot"]
