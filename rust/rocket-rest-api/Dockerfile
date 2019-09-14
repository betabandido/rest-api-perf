FROM rust:1.37-slim AS builder
RUN rustup update nightly
RUN rustup default nightly
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:stable-slim
COPY --from=builder /usr/local/cargo/bin/rocket-rest-api /
CMD ["/rocket-rest-api"]
