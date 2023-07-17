# SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
#
# SPDX-License-Identifier: MIT

.PHONY: test

-include nairobi.env

BIN_DIR:=$$PWD/bin
TARGET_DIR=$$PWD/target
CARGO_BIN_PATH:=$$HOME/.cargo/bin

install:
	cargo install tezos-smart-rollup-installer
	cd $(CARGO_BIN_PATH) \
		&& wget -c https://github.com/WebAssembly/binaryen/releases/download/version_111/binaryen-version_111-x86_64-linux.tar.gz -O - | tar -xzv binaryen-version_111/bin/wasm-opt --strip-components 2 \
		&& wget -c https://github.com/WebAssembly/wabt/releases/download/1.0.31/wabt-1.0.31-ubuntu.tar.gz -O - | tar -xzv wabt-1.0.31/bin/wasm-strip wabt-1.0.31/bin/wasm2wat --strip-components 2

build-kernel:
	RUSTC_BOOTSTRAP=1 cargo build --package $(PACKAGE) \
		--target wasm32-unknown-unknown \
		--release \
		-Z sparse-registry \
		-Z avoid-dev-deps
	wasm-strip -o $(BIN_DIR)/$(PACKAGE).wasm $(TARGET_DIR)/wasm32-unknown-unknown/release/$(PACKAGE).wasm

check-kernel:
	RUSTC_BOOTSTRAP=1 cargo build --package $(PACKAGE) \
		--no-default-features \
		--target wasm32-unknown-unknown \
		-Z avoid-dev-deps
	wasm2wat -o $(BIN_DIR)/$(PACKAGE).wat $(TARGET_DIR)/wasm32-unknown-unknown/debug/$(PACKAGE).wasm
	grep -nE 'f(32|64)\.' $(BIN_DIR)/$(PACKAGE).wat || true

debug-kernel:
	cargo build --package $(PACKAGE) \
		--target wasm32-unknown-unknown \
		--profile release \
		--target-dir $(TARGET_DIR)/repl
	wasm-strip -o $(BIN_DIR)/$(PACKAGE)_debug.wasm $(TARGET_DIR)/repl/wasm32-unknown-unknown/release/$(PACKAGE).wasm
	docker run --rm -it \
		--name wasm-repl \
		--entrypoint=/usr/local/bin/octez-smart-rollup-wasm-debugger \
		-v $(BIN_DIR):/home/bin \
		tezos/tezos:$(OCTEZ_TAG) \
		/home/bin/$(PACKAGE)_debug.wasm --inputs /home/bin/inputs.json

build-installer:
	smart-rollup-installer get-reveal-installer \
		--upgrade-to $(BIN_DIR)/$(PACKAGE).wasm \
		--output $(BIN_DIR)/$(PACKAGE)_installer.wasm \
		--preimages-dir $(BIN_DIR)/wasm_2_0_0

build-operator:
	mkdir $(BIN_DIR) || true
	$(MAKE) build-kernel PACKAGE=$(PACKAGE)
	$(MAKE) build-installer PACKAGE=$(PACKAGE)

build-facade:
	mkdir $(BIN_DIR) || true
	RUSTC_BOOTSTRAP=1 cargo build --package tezos_node \
		--release \
		-Z sparse-registry \
		-Z avoid-dev-deps
	cp $(TARGET_DIR)/release/tezos-node $(BIN_DIR)/tezos-node

test:
	RUSTC_BOOTSTRAP=1 RUST_BACKTRACE=1 cargo test -Z sparse-registry --no-fail-fast --tests -- --nocapture

nextest:
	RUST_LIB_BACKTRACE=1 cargo nextest run --tests

image-facade:
	docker build -t tot/facade:latest --file ./build/facade/Dockerfile .

image-operator:
	docker build -t tot/operator:${PACKAGE}_$(OCTEZ_TAG) --file ./build/operator/Dockerfile.local \
		--build-arg OCTEZ_TAG=$(OCTEZ_TAG) \
		--build-arg OCTEZ_PROTO=$(OCTEZ_PROTO) \
		--build-arg PACKAGE=$(PACKAGE) \
		.

run-operator:
	$(MAKE) build-operator PACKAGE=$(PACKAGE)
	$(MAKE) image-operator OCTEZ_TAG=$(OCTEZ_TAG) OCTEZ_PROTO=$(OCTEZ_PROTO) PACKAGE=$(PACKAGE)
	docker stop operator || true
	docker run --rm -it \
		--name operator \
		--entrypoint=/bin/sh \
		-v $$PWD/.tezos-client:/root/.tezos-client/ \
		-v ${PACKAGE}_operator:/root/.tezos-smart-rollup-node \
		-v $(BIN_DIR):/root/bin -p 127.0.0.1:8932:8932 \
		-e NETWORK=$(NETWORK) \
		tot/operator:${PACKAGE}_$(OCTEZ_TAG)

run-tezos-operator:
	$(MAKE) run-operator PACKAGE=tezos_kernel

run-sapling-operator:
	$(MAKE) run-operator PACKAGE=sapling_kernel

run-facade:
	$(MAKE) build-facade
	RUST_BACKTRACE=1 RUST_LOG=debug $(BIN_DIR)/tezos-node
