.PHONY: test build

build-wasm-repl:
	docker build -t tezos-bin ./build/octez-wasm-repl
	container_id=$$(docker create tezos-bin)
	docker cp "$$container_id:/octez-wasm-repl" ./bin/octez-wasm-repl
	docker rm "$$container_id"

install-wasm-opt:
	cd bin && wget -c https://github.com/WebAssembly/binaryen/releases/download/version_111/binaryen-version_111-x86_64-linux.tar.gz -O - | tar -xzv binaryen-version_111/bin/wasm-opt --strip-components 2

install-wasm-strip:
	cd bin && wget -c https://github.com/WebAssembly/wabt/releases/download/1.0.31/wabt-1.0.31-ubuntu.tar.gz -O - | tar -xzv wabt-1.0.31/bin/wasm-strip --strip-components 2

install:
	$(MAKE) install-wasm-opt
	$(MAKE) install-wasm-strip

build-tez-kernel:
	cargo build --package tez_kernel --target wasm32-unknown-unknown --release
	./bin/wasm-opt -Oz -o ./bin/tez_kernel.wasm ./target/wasm32-unknown-unknown/release/tez_kernel.wasm
	./bin/wasm-opt -Oz -o ./bin/genesis_kernel.wasm ./target/wasm32-unknown-unknown/release/genesis_kernel.wasm
	./bin/wasm-strip ./bin/genesis_kernel.wasm

build-genesis-kernel:
	cargo build --package genesis_kernel --target wasm32-unknown-unknown --release
	./bin/wasm-strip ./bin/tez_kernel.wasm

build-dac-coder:
	cargo build --package dac_coder --release
	cp ./target/release/dac-coder ./bin/dac-coder

build:
	$(MAKE) build-tez-kernel
	$(MAKE) build-genesis-kernel
	$(MAKE) build-dac-coder

test:
	RUST_BACKTRACE=1 cargo test --lib test -- --nocapture

run-genesis:
	./bin/octez-wasm-repl ./bin/genesis_kernel.wasm --inputs ./test/input.json

run-tez-kernel:
	./bin/octez-wasm-repl ./bin/tez_kernel.wasm --inputs ./test/input.json
