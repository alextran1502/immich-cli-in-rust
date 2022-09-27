# 1. This tells docker to use the Rust official image
FROM rust:1.63 as builder

WORKDIR /usr/src/immich-cli
COPY . .
RUN cargo install --path .

FROM debian:buster-slim

RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/immich-cli /usr/local/bin/immich-cli

WORKDIR /import
ENTRYPOINT ["immich-cli"]
# CMD ["immich-cli"]