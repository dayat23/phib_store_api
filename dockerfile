FROM rust:1.77.1-slim as builder

WORKDIR /usr/src/

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/app

COPY --from=builder /usr/src/config /usr/app/config
COPY --from=builder /usr/src/target/release/phib_store_api-cli /usr/app/phib_store_api-cli

ENTRYPOINT ["/usr/app/phib_store_api-cli"]