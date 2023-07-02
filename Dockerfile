FROM rust:1.70 AS builder
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt install -y openssl
COPY --from=builder ./target/release/clubvent ./target/release/clubvent
CMD ["/target/release/clubvent"]