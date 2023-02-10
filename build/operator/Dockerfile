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
RUN apk --no-cache add binutils gcc gmp libgmpxx hidapi libc-dev libev libffi sudo wget
RUN wget "https://raw.githubusercontent.com/zcash/zcash/master/zcutil/fetch-params.sh" \
  && export OSTYPE=linux \
  && sed '/SAPLING_SPROUT_GROTH16_NAME/d; /progress/d; /retry-connrefused/d' fetch-params.sh | sh \
  && rm fetch-params.sh
COPY --from=octez /usr/local/bin/octez-smart-rollup-node-PtMumbai /usr/bin/octez-smart-rollup-node
COPY --from=octez /usr/local/bin/octez-client /usr/bin/octez-client
COPY --from=builder /build/.bin/wasm_2_0_0/ /root/.tezos-sc-rollup-node/wasm_2_0_0/
COPY --from=builder /build/.bin/installer.wasm /root/.tezos-sc-rollup-node/kernel.wasm
COPY ./build/operator/entrypoint.sh .
RUN chmod +x entrypoint.sh
ARG NETWORK="mumbainet"
ENV NETWORK=${NETWORK}
ENTRYPOINT [ "./entrypoint.sh" ]