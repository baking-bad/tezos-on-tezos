# SPDX-FileCopyrightText: 2020 TQ Tezos <https://tqtezos.com/>
#
# SPDX-License-Identifier: LicenseRef-MIT-TQ
FROM alpine:3.12 as builder
RUN apk update
RUN apk --no-cache --virtual add rsync git m4 build-base patch unzip \
  bubblewrap wget pkgconfig gmp-dev libev-dev hidapi-dev eudev-dev perl opam libusb-dev bash \
  autoconf automake libtool linux-headers cargo libffi-dev zlib-static coreutils
COPY ./build/octez/build-libusb-and-hidapi.sh /build-libusb-and-hidapi.sh
RUN /build-libusb-and-hidapi.sh
COPY ./build/octez/build-rust.sh /build-rust.sh
RUN /build-rust.sh
ARG OCTEZ_VERSION=master
COPY ./build/octez/build-tezos-executables.sh /build-tezos-executables.sh
RUN /build-tezos-executables.sh ${OCTEZ_VERSION}

FROM alpine:3.12 as target
COPY --from=builder /tezos/octez-wasm-repl-alpha /usr/bin/octez-wasm-repl
COPY --from=builder /tezos/octez-sc-rollup-node-alpha /usr/bin/octez-sc-rollup-node
COPY --from=builder /tezos/octez-client-alpha /usr/bin/octez-client