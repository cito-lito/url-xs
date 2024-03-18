# Leveraging the pre-built Docker images with
# cargo-chef and the Rust toolchain
FROM lukemathwalker/cargo-chef:latest-rust-1.75.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --recipe-path recipe.json

COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM rust:1.75-slim AS template-rust
COPY --from=builder /app/target/release/url-xs /usr/local/bin
ENTRYPOINT ["/usr/local/bin/url-xs"]