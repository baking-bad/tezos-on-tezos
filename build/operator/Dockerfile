# SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
#
# SPDX-License-Identifier: MIT

ARG OCTEZ_TAG
FROM tezos/tezos:${OCTEZ_TAG} AS octez

FROM rust:slim-buster AS builder
RUN apt update && apt install -y wget make libc-dev
RUN rustup target add wasm32-unknown-unknown
WORKDIR /build
COPY Makefile ./
RUN make install CARGO_BIN_PATH=/usr/local/cargo/bin
COPY . .
ARG PACKAGE
RUN make build-operator PACKAGE=${PACKAGE}

FROM alpine:3.15 AS rollup
RUN apk --no-cache add binutils gcc gmp libgmpxx hidapi libc-dev libev libffi sudo
ARG OCTEZ_PROTO
#ARG RELEASE="auto-release"
#RUN wget "https://github.com/serokell/tezos-packaging/releases/download/$RELEASE/octez-smart-rollup-node-$OCTEZ_PROTO" -O "/usr/bin/octez-smart-rollup-node" \
#  && chmod +x /usr/bin/octez-smart-rollup-node
#RUN wget "https://github.com/serokell/tezos-packaging/releases/download/$RELEASE/octez-client" -O "/usr/bin/octez-client" \
#  && chmod +x /usr/bin/octez-client
COPY --from=octez /usr/local/bin/octez-smart-rollup-node-${OCTEZ_PROTO} /usr/bin/octez-smart-rollup-node
COPY --from=octez /usr/local/bin/octez-client /usr/bin/octez-client
COPY --from=builder /build/bin/wasm_2_0_0/ /root/wasm_2_0_0/
ARG PACKAGE
COPY --from=builder /build/bin/${PACKAGE}_installer.wasm /root/kernel.wasm
COPY ./build/operator/entrypoint.sh .
RUN chmod +x entrypoint.sh && ln ./entrypoint.sh /usr/bin/operator
ENTRYPOINT [ "./entrypoint.sh" ]