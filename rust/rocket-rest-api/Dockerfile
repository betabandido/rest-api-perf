ARG APP_NAME=rocket_rest_api

# build image
FROM rust:1.37-slim AS build

ARG APP_NAME

RUN rustup update nightly
RUN rustup default nightly

RUN USER=root cargo new --bin ${APP_NAME}
WORKDIR /${APP_NAME}

# build dependencies only
COPY ./Cargo.lock .
COPY ./Cargo.toml .
RUN cargo build --release
RUN rm src/*.rs
RUN rm ./target/release/deps/${APP_NAME}*

# build application
COPY ./src ./src
RUN cargo build --release

# release image
FROM debian:stable-slim

ARG APP_NAME

COPY --from=build /${APP_NAME}/target/release/${APP_NAME} /app
CMD ["/app"]
