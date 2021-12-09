FROM debian:bookworm-slim
WORKDIR /usr/bin
ADD https://github.com/gear-tech/gear/releases/download/build/gear-node ./
RUN chmod +x ./gear-node
RUN apt-get update -y && apt-get install -y ca-certificates
CMD ["gear-node"]
