.PHONY: build test

TAG:=monday

# https://teztnets.xyz/dailynet-about
DAILY_TAG=master_7e51d27c_20221220201529
DAILY_NETWORK=dailynet-2022-12-21

# https://teztnets.xyz/mondaynet-about
MONDAY_TAG=master_69656b8a_20230211074400
MONDAY_NETWORK=mondaynet-2023-02-13

# https://teztnets.xyz/mumbainet-about
MUMBAI_TAG=v16.0-rc1
MUMBAI_NETWORK=mumbainet

install:
	cd ~/.cargo/bin \
		&& wget -c https://github.com/WebAssembly/binaryen/releases/download/version_111/binaryen-version_111-x86_64-linux.tar.gz -O - | tar -xzv binaryen-version_111/bin/wasm-opt --strip-components 2 \
		&& wget -c https://github.com/WebAssembly/wabt/releases/download/1.0.31/wabt-1.0.31-ubuntu.tar.gz -O - | tar -xzv wabt-1.0.31/bin/wasm-strip wabt-1.0.31/bin/wasm2wat --strip-components 2

build-kernel:
	RUSTC_BOOTSTRAP=1 cargo build --package tez_kernel --target wasm32-unknown-unknown --release -Z sparse-registry
	wasm-strip -o ./.bin/tez_kernel.wasm ./target/wasm32-unknown-unknown/release/tez_kernel.wasm
	# wasm-opt -Oz -o ./.bin/tez_kernel.wasm ./target/wasm32-unknown-unknown/release/tez_kernel.wasm

build-installer:
	RUSTC_BOOTSTRAP=1 cargo build --package installer --target wasm32-unknown-unknown --release -Z sparse-registry
	wasm-strip -o ./.bin/installer.wasm ./target/wasm32-unknown-unknown/release/installer.wasm
	# wasm-opt -Oz -o ./.bin/installer.wasm ./target/wasm32-unknown-unknown/release/installer.wasm

build-dac-codec:
	RUSTC_BOOTSTRAP=1 cargo build --package dac_codec --release -Z sparse-registry 
	cp ./target/release/dac-codec ./.bin/dac-codec

build-node:
	mkdir .bin || true
	RUSTC_BOOTSTRAP=1 cargo build --package tezos_node --release -Z sparse-registry 
	cp ./target/release/tezos-node ./.bin/tezos-node

pages:
	rm -rf ./.bin/wasm_2_0_0
	./.bin/dac-codec -o ./.bin/wasm_2_0_0 ./.bin/tez_kernel.wasm

build:
	mkdir .bin || true
	$(MAKE) build-kernel
	$(MAKE) build-dac-codec
	$(MAKE) pages
	$(MAKE) build-installer

test:
	RUSTC_BOOTSTRAP=1 RUST_BACKTRACE=1 cargo test -Z sparse-registry --no-fail-fast --tests -- --nocapture

nextest:
	RUST_LIB_BACKTRACE=1 cargo nextest run --tests

trace:
# TODO: pass test suite name
	RUST_LIB_BACKTRACE=1 cargo test --jobs 1 --no-fail-fast --test e2e --features trace -- --nocapture --test-threads=1 e2e_abs_00

image-facade:
	docker build -t ghcr.io/baking-bad/tz-rollup-facade:$(TAG) --file ./build/facade/Dockerfile .

image-operator:
	docker build -t ghcr.io/baking-bad/tz-rollup-operator:$(TAG) --build-arg OCTEZ_TAG=$(OCTEZ_TAG) --build-arg OCTEZ_PROTO=$(OCTEZ_PROTO) --build-arg NETWORK=$(NETWORK) --file ./build/operator/Dockerfile.local .

image-operator-monday:
	$(MAKE) image-operator TAG=monday OCTEZ_TAG=$(MONDAY_TAG) OCTEZ_PROTO=alpha NETWORK=$(MONDAY_NETWORK)

image-operator-mumbai:
	$(MAKE) image-operator TAG=mumbai OCTEZ_TAG=$(MUMBAI_TAG) OCTEZ_PROTO=PtMumbai NETWORK=$(MUMBAI_NETWORK)

generate-keypair:
	docker run --rm -v $$PWD/.tezos-client:/root/.tezos-client/ -v rollup-node-$(TAG):/root/.tezos-smart-rollup-node ghcr.io/baking-bad/tz-rollup-operator:$(TAG) generate-keypair

originate-rollup:
	docker stop tz-rollup-operator || true
	docker rm tz-rollup-operator || true
	docker volume rm rollup-node-$(TAG) || true
	docker run --rm -v $$PWD/.tezos-client:/root/.tezos-client/ -v rollup-node-$(TAG):/root/.tezos-smart-rollup-node ghcr.io/baking-bad/tz-rollup-operator:$(TAG) originate-rollup

rollup-node:
	docker stop tz-rollup-operator || true
	docker run --rm --name tz-rollup-operator -d -v $$PWD/.tezos-client:/root/.tezos-client/ -v $$PWD/.logs:/root/logs/ -v rollup-node-$(TAG):/root/.tezos-smart-rollup-node -p 127.0.0.1:8932:8932 ghcr.io/baking-bad/tz-rollup-operator:$(TAG) rollup-node --debug
	docker logs tz-rollup-operator -f 2>&1 | grep smart_rollup_node

populate-inbox:
	docker run --rm -v $$PWD/.tezos-client:/root/.tezos-client/ -v $$PWD/.bin:/root/.bin ghcr.io/baking-bad/tz-rollup-operator:$(TAG) populate-inbox /root/.bin/messages.json

operator-shell:
	docker run --rm -it --entrypoint=/bin/bash -v $$PWD/.tezos-client:/root/.tezos-client/ -v rollup-node-$(TAG):/root/.tezos-smart-rollup-node ghcr.io/baking-bad/tz-rollup-operator:$(TAG)

wat:
	cargo build --package tez_kernel --target wasm32-unknown-unknown
	wasm2wat -o ./.bin/kernel.wat ./target/wasm32-unknown-unknown/debug/tez_kernel.wasm

debug:
	cargo build --package tez_kernel --target wasm32-unknown-unknown --profile release --target-dir ./target/repl
	wasm-strip -o ./.bin/debug_kernel.wasm ./target/repl/wasm32-unknown-unknown/release/tez_kernel.wasm
	docker run --rm -it --entrypoint=/bin/sh --name wasm-repl -v $$PWD/.bin:/root/.bin tezos/tezos:$(MONDAY_TAG) /usr/local/bin/octez-smart-rollup-wasm-debugger /root/.bin/debug_kernel.wasm --inputs /root/.bin/inputs.json

daily:
	$(MAKE) build
	$(MAKE) image-operator TAG=daily OCTEZ_TAG=$(DAILY_TAG) NETWORK=$(DAILY_NETWORK) OCTEZ_PROTO=alpha
	$(MAKE) originate-rollup TAG=daily
	$(MAKE) rollup-node TAG=daily

monday:
	$(MAKE) build
	$(MAKE) image-operator-monday
	$(MAKE) originate-rollup TAG=monday
	$(MAKE) rollup-node TAG=monday

mumbai:
	$(MAKE) build
	$(MAKE) image-operator-mumbai
	$(MAKE) originate-rollup TAG=mumbai
	$(MAKE) rollup-node TAG=mumbai

facade:
	$(MAKE) build-node
	$(MAKE) image-facade
