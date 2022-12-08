FROM ghcr.io/baking-bad/scoru-kit:master AS bin

FROM rust:slim-buster AS builder
RUN apt update && apt install -y wget make libc-dev
RUN rustup target add wasm32-unknown-unknown
WORKDIR /usr/bin
RUN wget -c https://github.com/WebAssembly/binaryen/releases/download/version_111/binaryen-version_111-x86_64-linux.tar.gz -O - | tar -xzv binaryen-version_111/bin/wasm-opt --strip-components 2
RUN wget -c https://github.com/WebAssembly/wabt/releases/download/1.0.31/wabt-1.0.31-ubuntu.tar.gz -O - | tar -xzv wabt-1.0.31/bin/wasm-strip --strip-components 2
WORKDIR /build
COPY . .
RUN make build

FROM alpine:3.15 as target
COPY --from=bin /usr/bin/octez-sc-rollup-node /usr/bin/octez-sc-rollup-node
COPY --from=bin /usr/bin/octez-client /usr/bin/octez-client
COPY --from=builder /build/.dac/ /rollup/wasm_2_0_0/
COPY --from=builder /build/.bin/genesis_kernel.wasm /rollup/kernel.wasm
RUN export KERNEL=$(xxd -p /rollup/kernel.wasm | tr -d '\n')