# Stage 1: Build
FROM rust:1 as builder

# Install musl target support
RUN apt-get update && \
    apt-get install -y musl-tools pkg-config libssl-dev && \
    rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage 2: Minimal runtime
FROM alpine:latest

RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust-rocket-sample /usr/local/bin/rust-rocket-sample
COPY --from=builder /app/.env .env
COPY --from=builder /app/Rocket.toml Rocket.toml

EXPOSE 8080

CMD ["/usr/local/bin/rust-rocket-sample"]
