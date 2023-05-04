FROM ubuntu:22.04
COPY target/release/cnnvd-provider /app/cnnvd-provider
RUN chmod +x /app/cnnvd-provider
WORKDIR /app
CMD ["./cnnvd-provider"]