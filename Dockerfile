# Stage 1: Build React frontend
FROM node:20 AS fe-builder

WORKDIR /app/fe

COPY fe/package*.json ./
RUN npm install

COPY fe/ ./

# Set the base URL during build
ARG REACT_APP_BASE_URL
ENV REACT_APP_BASE_URL=$REACT_APP_BASE_URL

RUN npm run build

# Stage 2: Build Rust backend
FROM rust:1 AS be-builder

RUN apt-get update && \
  apt-get install -y musl-tools pkg-config libssl-dev && \
  rustup target add x86_64-unknown-linux-musl

WORKDIR /app

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage 3: Runtime
FROM alpine:latest

RUN apk add --no-cache ca-certificates

# Copy Rocket backend binary and config
COPY --from=be-builder /app/target/x86_64-unknown-linux-musl/release/portfolio-cms /usr/local/bin/portfolio-cms
COPY --from=be-builder /app/Rocket.toml Rocket.toml

# Copy React build output to folder Rocket will serve (e.g., "public")
COPY --from=fe-builder /app/fe/build /public

EXPOSE 8080

CMD ["/usr/local/bin/portfolio-cms"]
