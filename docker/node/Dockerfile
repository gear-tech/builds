FROM debian:bookworm-slim
RUN apt-get update -y && apt-get install -y ca-certificates
ADD https://github.com/gear-tech/gear/releases/download/build/gear-node /usr/bin/
RUN chmod +x /usr/bin/gear-node
EXPOSE 9933
EXPOSE 9944
EXPOSE 9165
EXPOSE 30333
CMD ["gear-node"]
