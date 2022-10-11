FROM debian:bookworm-slim
RUN apt-get update -y && apt-get install -y ca-certificates
ADD https://github.com/gear-tech/gear/releases/download/build/gear /usr/bin/
RUN chmod +x /usr/bin/gear
RUN gear build-spec --dev > /root/workshop-spec.json
RUN sed -i 's/"name": "Development"/"name": "Workshop"/' /root/workshop-spec.json
EXPOSE 9933
EXPOSE 9944
EXPOSE 9165
EXPOSE 30333
CMD ["gear", "--chain", "/root/workshop-spec.json", "--alice", "--tmp", "--unsafe-ws-external", "--unsafe-rpc-external", "--rpc-methods", "Unsafe", "--rpc-cors", "all"]
