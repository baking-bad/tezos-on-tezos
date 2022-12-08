FROM ghcr.io/baking-bad/scoru-kit:master AS builder
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh \
    && rustup target add wasm32-unknown-unknown
WORKDIR /build
COPY . .
RUN make build

FROM alpine:3.12 as target
COPY --from=builder /usr/bin/octez-sc-rollup-node /usr/bin/octez-sc-rollup-node
COPY --from=builder /usr/bin/octez-client /usr/bin/octez-client
COPY --from=builder /build/.dac/ /rollup/wasm_2_0_0/
COPY --from=builder /build/.bin/genesis_kernel.wasm /rollup/kernel.wasm
