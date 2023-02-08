FROM rust:slim-buster AS builder
RUN apt update && apt install -y wget make libc-dev
WORKDIR /usr/bin
RUN wget -c https://github.com/WebAssembly/binaryen/releases/download/version_111/binaryen-version_111-x86_64-linux.tar.gz -O - | tar -xzv binaryen-version_111/bin/wasm-opt --strip-components 2
RUN wget -c https://github.com/WebAssembly/wabt/releases/download/1.0.31/wabt-1.0.31-ubuntu.tar.gz -O - | tar -xzv wabt-1.0.31/bin/wasm-strip --strip-components 2
WORKDIR /build
COPY . .
RUN make build-node

FROM rust:slim-buster AS facade
COPY --from=builder /build/.bin/tezos-node /usr/bin/tezos-node
ENTRYPOINT [ "/usr/bin/tezos-node" ]
