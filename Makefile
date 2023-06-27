# SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
#
# SPDX-License-Identifier: MIT

.PHONY: build test

TAG:=
OCTEZ_TAG:=
OCTEZ_PROTO:=
NETWORK:=
CARGO_BIN_PATH:=$$HOME/.cargo/bin

env-mumbainet:
ifeq ($(NETWORK), mumbainet)
	@echo "NETWORK is already set to 'mumbainet'"
else
#	$(eval OCTEZ_TAG := $(shell curl -s https://teztnets.xyz/teztnets.json | jq -r ".mumbainet.git_ref"))
	$(eval OCTEZ_TAG := v17.0)
	$(eval OCTEZ_PROTO := $(shell curl -s https://teztnets.xyz/teztnets.json | jq -r ".mumbainet.last_baking_daemon"))
	$(eval NETWORK := mumbainet)
	$(eval TAG := mumbai)
	@echo "OCTEZ_TAG is now set to: $(OCTEZ_TAG)"
	@echo "OCTEZ_PROTO is now set to: $(OCTEZ_PROTO)"
	@echo "NETWORK is now set to: $(NETWORK)"
	@echo "TAG is now set to: $(TAG)"
endif

env-mondaynet:
ifeq ($(NETWORK), mondaynet)
	@echo "NETWORK is already set to 'mondaynet'"
else
	$(eval OCTEZ_TAG := $(shell curl -s https://teztnets.xyz/teztnets.json | jq -r '. | to_entries | map(select(.key | startswith("monday"))) | map(.value.docker_build)[0]'))
	$(eval OCTEZ_PROTO := $(shell curl -s https://teztnets.xyz/teztnets.json | jq -r '. | to_entries | map(select(.key | startswith("monday"))) | map(.value.last_baking_daemon)[0]')
	$(eval NETWORK := mondaynet)
	$(eval TAG := monday)
	@echo "OCTEZ_TAG is now set to: $(OCTEZ_TAG)"
	@echo "OCTEZ_PROTO is now set to: $(OCTEZ_PROTO)"
	@echo "NETWORK is now set to: $(NETWORK)"
	@echo "TAG is now set to: $(TAG)"
endif

install:
	cargo install tezos-smart-rollup-installer
	cd $(CARGO_BIN_PATH) \
		&& wget -c https://github.com/WebAssembly/binaryen/releases/download/version_111/binaryen-version_111-x86_64-linux.tar.gz -O - | tar -xzv binaryen-version_111/bin/wasm-opt --strip-components 2 \
		&& wget -c https://github.com/WebAssembly/wabt/releases/download/1.0.31/wabt-1.0.31-ubuntu.tar.gz -O - | tar -xzv wabt-1.0.31/bin/wasm-strip wabt-1.0.31/bin/wasm2wat --strip-components 2


build-kernel:
	RUSTC_BOOTSTRAP=1 cargo build --package tezos_kernel --target wasm32-unknown-unknown --release -Z sparse-registry -Z avoid-dev-deps
	wasm-strip -o ./.bin/tezos_kernel.wasm ./target/wasm32-unknown-unknown/release/tezos_kernel.wasm
	# wasm-opt -Oz -o ./.bin/tezos_kernel.wasm ./target/wasm32-unknown-unknown/release/tezos_kernel.wasm

build-installer:
	smart-rollup-installer get-reveal-installer \
		--upgrade-to ./.bin/tezos_kernel.wasm \
		--output ./.bin/installer.wasm \
		--preimages-dir ./.bin/wasm_2_0_0

build-facade:
	mkdir .bin || true
	RUSTC_BOOTSTRAP=1 cargo build --package tezos_node --release -Z sparse-registry -Z avoid-dev-deps
	cp ./target/release/tezos-node ./.bin/tezos-node

build-operator:
	mkdir .bin || true
	$(MAKE) build-kernel
	$(MAKE) build-installer

test:
	RUSTC_BOOTSTRAP=1 RUST_BACKTRACE=1 cargo test -Z sparse-registry --no-fail-fast --tests -- --nocapture

nextest:
	RUST_LIB_BACKTRACE=1 cargo nextest run --tests

trace:
# TODO: pass test suite name
	RUST_LIB_BACKTRACE=1 cargo test --jobs 1 --no-fail-fast --test e2e --features trace -- --nocapture --test-threads=1 e2e_abs_00

image-facade:
	docker build -t ghcr.io/baking-bad/tz-rollup-facade:latest --file ./build/facade/Dockerfile.local .

image-operator:
	docker build -t ghcr.io/baking-bad/tz-rollup-operator:$(TAG) --build-arg OCTEZ_TAG=$(OCTEZ_TAG) --build-arg OCTEZ_PROTO=$(OCTEZ_PROTO) --build-arg NETWORK=$(NETWORK) --file ./build/operator/Dockerfile.local .

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
	docker run --rm -it --entrypoint=/bin/sh -v $$PWD/.tezos-client:/root/.tezos-client/ -v rollup-node-$(TAG):/root/.tezos-smart-rollup-node ghcr.io/baking-bad/tz-rollup-operator:$(TAG)

wat:
	cargo build --package tezos_kernel --target wasm32-unknown-unknown
	wasm2wat -o ./.bin/kernel.wat ./target/wasm32-unknown-unknown/debug/tezos_kernel.wasm
	# check if there's no floating point calc
	grep -nE 'f(32|64)\.' ./.bin/kernel.wat || true

debug: env-mondaynet
	cargo build --package tezos_kernel --target wasm32-unknown-unknown --profile release --target-dir ./target/repl
	wasm-strip -o ./.bin/debug_kernel.wasm ./target/repl/wasm32-unknown-unknown/release/tezos_kernel.wasm
	docker run --rm -it --entrypoint=/usr/local/bin/octez-smart-rollup-wasm-debugger --name wasm-repl -v $$PWD/.bin:/home/.bin tezos/tezos:$(TAG) /home/.bin/debug_kernel.wasm --inputs /home/.bin/inputs.json

shell-monday: env-mondaynet
	$(MAKE) operator-shell TAG=$(TAG)

shell-mumbai: env-mumbainet
	$(MAKE) operator-shell TAG=$(TAG)

image-operator-monday: env-mondaynet
	$(MAKE) image-operator TAG=$(TAG) OCTEZ_TAG=$(OCTEZ_TAG) OCTEZ_PROTO=$(OCTEZ_PROTO) NETWORK=$(NETWORK)

image-operator-mumbai: env-mumbainet
	$(MAKE) image-operator TAG=$(TAG) OCTEZ_TAG=$(OCTEZ_TAG) OCTEZ_PROTO=$(OCTEZ_PROTO) NETWORK=$(NETWORK)

operator-monday: env-mondaynet
	$(MAKE) build-operator
	$(MAKE) image-operator TAG=$(TAG) OCTEZ_TAG=$(OCTEZ_TAG) OCTEZ_PROTO=$(OCTEZ_PROTO) NETWORK=$(NETWORK)
	$(MAKE) originate-rollup TAG=$(TAG)
	$(MAKE) rollup-node TAG=$(TAG)

operator-mumbai: env-mumbainet
	$(MAKE) build-operator
	$(MAKE) image-operator TAG=$(TAG) OCTEZ_TAG=$(OCTEZ_TAG) OCTEZ_PROTO=$(OCTEZ_PROTO) NETWORK=$(NETWORK)
	$(MAKE) originate-rollup TAG=$(TAG)
	$(MAKE) rollup-node TAG=$(TAG)

facade:
	$(MAKE) build-facade
	RUST_BACKTRACE=1 RUST_LOG=debug ./.bin/tezos-node
