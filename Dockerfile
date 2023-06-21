FROM rust:1.70 AS builder
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder ./target/release/bookclub_bot ./target/release/bookclub_bot
CMD ["/target/release/bookclub_bot"]