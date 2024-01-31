# syntax=docker/dockerfile:1

FROM rust:1.75-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:12.4-slim
WORKDIR /app
COPY --from=builder /app/target/release/docker-calculator .
CMD ["./docker-calculator"]
EXPOSE 8080