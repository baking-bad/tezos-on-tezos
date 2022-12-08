.PHONY: build

build-scoru-kit:
	DOCKER_BUILDKIT=1 docker build -t ghcr.io/baking-bad/scoru-kit --file ./build/scoru-kit/Dockerfile .

build-tez-rollup:
	DOCKER_BUILDKIT=1 docker build -t ghcr.io/baking-bad/tez-rollup --file ./build/tez-rollup/Dockerfile .

install-scoru-kit:
	mkdir .bin || true 
	docker pull ghcr.io/baking-bad/scoru-kit
	bash ./build/scoru-kit/get-artifacts.sh

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
	./.bin/dac-coder -o ./.dac ./.bin/tez_kernel.wasm

build:
	mkdir .bin || true
	$(MAKE) build-tez-kernel
	$(MAKE) build-dac-coder
	$(MAKE) pages
	$(MAKE) build-genesis-kernel

test:
	RUSTC_BOOTSTRAP=1 RUST_BACKTRACE=1 cargo test -Z sparse-registry --lib test -- --nocapture
