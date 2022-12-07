FROM ghcr.io/baking-bad/octez:master AS octez

FROM rust AS builder

WORKDIR /src
COPY . .
RUN make install && make build
RUN mv /src/dac/* /rollup/wasm_2_0_0/