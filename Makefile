.PHONY: build

install:
	cd ~/.cargo/bin \
		&& wget -c https://github.com/WebAssembly/binaryen/releases/download/version_111/binaryen-version_111-x86_64-linux.tar.gz -O - | tar -xzv binaryen-version_111/bin/wasm-opt --strip-components 2 \
		&& wget -c https://github.com/WebAssembly/wabt/releases/download/1.0.31/wabt-1.0.31-ubuntu.tar.gz -O - | tar -xzv wabt-1.0.31/bin/wasm-strip --strip-components 2

build-tez-kernel:
	RUSTC_BOOTSTRAP=1 cargo build --package tez_kernel --target wasm32-unknown-unknown --release -Z sparse-registry
	wasm-opt -Oz -o ./.bin/tez_kernel.wasm ./target/wasm32-unknown-unknown/release/tez_kernel.wasm
	wasm-strip ./.bin/tez_kernel.wasm

build-genesis-kernel:
	RUSTC_BOOTSTRAP=1 cargo build --package genesis_kernel --target wasm32-unknown-unknown --release -Z sparse-registry
	wasm-opt -Oz -o ./.bin/genesis_kernel.wasm ./target/wasm32-unknown-unknown/release/genesis_kernel.wasm
	wasm-strip ./.bin/genesis_kernel.wasm

build-dac-coder:
	RUSTC_BOOTSTRAP=1 cargo build --package dac_coder --release -Z sparse-registry 
	cp ./target/release/dac-coder ./.bin/dac-coder

pages:
	rm -rf ./.bin/wasm_2_0_0
	./.bin/dac-coder -o ./.bin/wasm_2_0_0 ./.bin/tez_kernel.wasm

build:
	mkdir .bin || true
	$(MAKE) build-tez-kernel
	$(MAKE) build-dac-coder
	$(MAKE) pages
	$(MAKE) build-genesis-kernel

test:
	RUSTC_BOOTSTRAP=1 RUST_BACKTRACE=1 cargo test -Z sparse-registry --lib test -- --nocapture

image:
	DOCKER_BUILDKIT=1 docker build -t ghcr.io/baking-bad/tz-rollup-operator --file ./build/Dockerfile.local .

operator:
	docker run --rm -it --entrypoint=/bin/sh -v $$PWD/.tezos-client:/root/.tezos-client/ -v rollup-node:/root/.rollup-node ghcr.io/baking-bad/tz-rollup-operator

run:
	docker run --rm --name tz-rollup-operator -d -v $$PWD/.tezos-client:/root/.tezos-client/ -v rollup-node:/root/.rollup-node -p 127.0.0.1:8932:8932 ghcr.io/baking-bad/tz-rollup-operator rollup-node