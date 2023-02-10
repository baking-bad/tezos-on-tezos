FROM rust:alpine3.16 AS builder
RUN apk add --no-cache musl-dev openssl-dev
WORKDIR /build
COPY . .
RUN RUSTC_BOOTSTRAP=1 cargo build --package tezos_node --release -Z sparse-registry

FROM alpine:3.16 AS facade
COPY --from=builder /build/target/release/tezos-node /usr/bin/tezos-node
ENTRYPOINT [ "/usr/bin/tezos-node" ]
