FROM rust:1.75.0-buster as builder

WORKDIR /usr/src/myapp

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --release

# runtime environment
FROM debian:buster-slim
WORKDIR /app

COPY --from=builder /usr/src/myapp/target/release/url-xs /app/

EXPOSE 3003
ENV RUST_LOG=info

CMD ["./url-xs"]
