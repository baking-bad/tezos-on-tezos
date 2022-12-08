# SPDX-FileCopyrightText: 2020 TQ Tezos <https://tqtezos.com/>
#
# SPDX-License-Identifier: LicenseRef-MIT-TQ
FROM alpine:3.12 as builder
RUN apk update
RUN apk --no-cache --virtual add rsync git m4 build-base patch unzip \
  bubblewrap wget pkgconfig gmp-dev libev-dev hidapi-dev eudev-dev perl opam libusb-dev bash \
  autoconf automake libtool linux-headers cargo libffi-dev zlib-static coreutils
COPY ./build/scoru-kit/build-libusb-and-hidapi.sh /build-libusb-and-hidapi.sh
RUN /build-libusb-and-hidapi.sh
COPY ./build/scoru-kit/build-rust.sh /build-rust.sh
RUN /build-rust.sh
ARG OCTEZ_VERSION=master
COPY ./build/scoru-kit/build-tezos-executables.sh /build-tezos-executables.sh
RUN /build-tezos-executables.sh ${OCTEZ_VERSION}
WORKDIR /wasm
RUN wget -c https://github.com/WebAssembly/binaryen/releases/download/version_111/binaryen-version_111-x86_64-linux.tar.gz -O - | tar -xzv binaryen-version_111/bin/wasm-opt --strip-components 2
RUN wget -c https://github.com/WebAssembly/wabt/releases/download/1.0.31/wabt-1.0.31-ubuntu.tar.gz -O - | tar -xzv wabt-1.0.31/bin/wasm-strip --strip-components 2

FROM alpine:3.12 as target
WORKDIR /usr/bin
COPY --from=builder /tezos/octez-wasm-repl-alpha ./octez-wasm-repl
COPY --from=builder /tezos/octez-sc-rollup-node-alpha ./octez-sc-rollup-node
COPY --from=builder /tezos/octez-client ./octez-client
COPY --from=builder /wasm/wasm-opt ./wasm-opt
COPY --from=builder --chown=root:root /wasm/wasm-strip ./wasm-strip
