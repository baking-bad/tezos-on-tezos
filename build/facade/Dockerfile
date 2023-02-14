FROM rust:alpine3.16 AS builder
RUN apk add --no-cache musl-dev openssl-dev
WORKDIR /build
COPY . .
RUN RUSTFLAGS='-C target-feature=-crt-static' RUSTC_BOOTSTRAP=1 cargo build --package tezos_node --release -Z sparse-registry

FROM alpine:3.16 AS facade
RUN apk --no-cache add gcc libc-dev
COPY --from=builder /build/target/release/tezos-node /usr/bin/tezos-node
ENV RUST_BACKTRACE=1
ENV RUST_LOG=debug
ENTRYPOINT [ "/usr/bin/tezos-node" ]
