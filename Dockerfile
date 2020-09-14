FROM rust:1.46.0-alpine AS builder
# build project
RUN apk add musl-dev
WORKDIR /usr/src/
RUN USER=root cargo new hello-world-operator
WORKDIR /usr/src/hello-world-operator
COPY Cargo.toml .
COPY src ./src
RUN cargo build --release

# Bundle Stage
FROM alpine as final
COPY --from=builder /usr/src/hello-world-operator/target/release/hello-world-operator .
CMD ["./hello-world-operator"]