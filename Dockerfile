FROM rust:1.70 AS builder
COPY . .
RUN cargo build --release

FROM alpine:latest
RUN apk update && apk add openssl
RUN apk add --no-cache ca-certificates && update-ca-certificates
COPY --from=builder ./target/release/clubvent ./target/release/clubvent
CMD ["/target/release/clubvent"]