ARG OCTEZ_TAG=v16.0-rc1
FROM tezos/tezos:${OCTEZ_TAG} AS octez

FROM rust:slim-buster AS builder
RUN apt update && apt install -y wget make libc-dev
RUN rustup target add wasm32-unknown-unknown
WORKDIR /usr/bin
RUN wget -c https://github.com/WebAssembly/binaryen/releases/download/version_111/binaryen-version_111-x86_64-linux.tar.gz -O - | tar -xzv binaryen-version_111/bin/wasm-opt --strip-components 2
RUN wget -c https://github.com/WebAssembly/wabt/releases/download/1.0.31/wabt-1.0.31-ubuntu.tar.gz -O - | tar -xzv wabt-1.0.31/bin/wasm-strip --strip-components 2
WORKDIR /build
COPY . .
RUN make build

FROM alpine:3.15 AS rollup
RUN apk --no-cache add binutils gcc gmp libgmpxx hidapi libc-dev libev libffi sudo
ARG OCTEZ_PROTO="PtMumbai"
#ARG RELEASE="auto-release"
#RUN wget "https://github.com/serokell/tezos-packaging/releases/download/$RELEASE/octez-smart-rollup-node-$OCTEZ_PROTO" -O "/usr/bin/octez-smart-rollup-node" \
#  && chmod +x /usr/bin/octez-smart-rollup-node
#RUN wget "https://github.com/serokell/tezos-packaging/releases/download/$RELEASE/octez-client" -O "/usr/bin/octez-client" \
#  && chmod +x /usr/bin/octez-client
COPY --from=octez /usr/local/bin/octez-smart-rollup-node-${OCTEZ_PROTO} /usr/bin/octez-smart-rollup-node
COPY --from=octez /usr/local/bin/octez-client /usr/bin/octez-client
COPY --from=builder /build/.bin/wasm_2_0_0/ /root/wasm_2_0_0/
COPY --from=builder /build/.bin/installer.wasm /root/kernel.wasm
COPY ./build/operator/entrypoint.sh .
RUN chmod +x entrypoint.sh
ARG NETWORK="mumbainet"
ENV NETWORK=${NETWORK}
ENTRYPOINT [ "./entrypoint.sh" ]