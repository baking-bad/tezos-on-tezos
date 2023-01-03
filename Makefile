.PHONY: build test

TAG:=monday

# https://teztnets.xyz/dailynet-about
DAILY_TAG=master_7e51d27c_20221220201529
DAILY_NETWORK=dailynet-2022-12-21

# https://teztnets.xyz/mondaynet-about
MONDAY_TAG=master_c08dbd3e_20221216223044
MONDAY_NETWORK=mondaynet-2022-12-19

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
	cargo nextest run --tests

trace:
# TODO: pass test suite name
	cargo test --jobs 1 --no-fail-fast --test tzt_iter --features trace -- --nocapture --test-threads=1

image-daily:
	docker build -t ghcr.io/baking-bad/tz-rollup-operator:daily --build-arg OCTEZ_TAG=$(DAILY_TAG) --build-arg NETWORK=$(DAILY_NETWORK) --file ./build/Dockerfile.local .

image-monday:
	docker build -t ghcr.io/baking-bad/tz-rollup-operator:monday --build-arg OCTEZ_TAG=$(MONDAY_TAG) --build-arg NETWORK=$(MONDAY_NETWORK) --file ./build/Dockerfile.local .

image:
	$(MAKE) image-$(TAG)

show-address:
	sudo cat .tezos-client/public_key_hashs

generate-keypair:
	docker run --rm -v $$PWD/.tezos-client:/root/.tezos-client/ -v rollup-node-$(TAG):/root/.tezos-sc-rollup-node ghcr.io/baking-bad/tz-rollup-operator:$(TAG) generate-keypair

originate-rollup:
	docker stop tz-rollup-operator || true
	docker rm tz-rollup-operator || true
	docker volume rm rollup-node-$(TAG) || true
	docker run --rm -v $$PWD/.tezos-client:/root/.tezos-client/ -v rollup-node-$(TAG):/root/.tezos-sc-rollup-node ghcr.io/baking-bad/tz-rollup-operator:$(TAG) originate-rollup

rollup-node:
	docker run --name tz-rollup-operator -d -v $$PWD/.tezos-client:/root/.tezos-client/ -v rollup-node-$(TAG):/root/.tezos-sc-rollup-node -p 127.0.0.1:8932:8932 ghcr.io/baking-bad/tz-rollup-operator:$(TAG) rollup-node
	docker logs tz-rollup-operator -f

populate-inbox:
	docker run --rm -v $$PWD/.tezos-client:/root/.tezos-client/ -v $$PWD/.bin:/root/.bin ghcr.io/baking-bad/tz-rollup-operator:$(TAG) populate-inbox /root/.bin/messages.json

operator-shell:
	docker run --rm -it --entrypoint=/bin/sh -v $$PWD/.tezos-client:/root/.tezos-client/ -v rollup-node-$(TAG):/root/.tezos-sc-rollup-node ghcr.io/baking-bad/tz-rollup-operator:$(TAG)

wat:
	cargo build --package tez_kernel --target wasm32-unknown-unknown
	wasm2wat -o ./.bin/kernel.wat ./target/wasm32-unknown-unknown/debug/tez_kernel.wasm

debug:
	docker build -t ghcr.io/baking-bad/tz-rollup-operator:debug --file ./build/Dockerfile.debug .
	cargo build --package tez_kernel --target wasm32-unknown-unknown --profile release --target-dir ./target/repl
	wasm-strip -o ./.bin/debug_kernel.wasm ./target/repl/wasm32-unknown-unknown/release/tez_kernel.wasm
	docker run --rm -it --name wasm-repl -v $$PWD/.bin:/root/.bin ghcr.io/baking-bad/tz-rollup-operator:debug wasm-repl /root/.bin/debug_kernel.wasm --inputs /root/.bin/inputs.json

daily:
	$(MAKE) build
	$(MAKE) image TAG=daily
	$(MAKE) originate-rollup TAG=daily
	$(MAKE) rollup-node TAG=daily

monday:
	$(MAKE) build
	$(MAKE) image TAG=monday
	$(MAKE) originate-rollup TAG=monday
	$(MAKE) rollup-node TAG=monday