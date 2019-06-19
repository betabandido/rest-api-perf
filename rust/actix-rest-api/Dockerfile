FROM rust:1.35-slim AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:stable-slim
COPY --from=builder /usr/local/cargo/bin/actix-rest-api /
CMD ["/actix-rest-api"]
