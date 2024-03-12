FROM clux/muslrust:nightly-2024-03-01 AS builder

WORKDIR /app

COPY ./api ./api
COPY ./claude ./claude
COPY ./deepl ./deepl
COPY ./env ./env
COPY ./translation ./translation
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY toolchain.toml toolchain.toml
COPY LICENSE LICENSE

ENV TARGET x86_64-unknown-linux-musl
RUN cargo build --release



FROM alpine:3 AS production

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/api /app/translation-api

CMD ["./translation-api"]
