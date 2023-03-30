# build dependencies
FROM rust:alpine3.17 as cacher
RUN apk add musl-dev
RUN apk add libressl-dev
RUN rustup default nightly
RUN cargo install cargo-chef

WORKDIR /app
COPY recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json


# build app
FROM rust:alpine3.17 as builder
RUN apk add musl-dev
RUN apk add libressl-dev
RUN rustup default nightly

WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

RUN cargo build --release


# runtime
FROM alpine:3.17

COPY --from=builder /app/target /app
WORKDIR /app

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["./release/bec_backend"]
