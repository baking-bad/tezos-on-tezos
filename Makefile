.PHONY: test build

build-wasm-repl:
	docker build -t tezos-bin ./build/octez-wasm-repl
	container_id=$$(docker create tezos-bin)
	docker cp "$$container_id:/octez-wasm-repl" ./bin/octez-wasm-repl
	docker rm "$$container_id"

install-wasm-opt:
	cd bin && wget -c https://github.com/WebAssembly/binaryen/releases/download/version_111/binaryen-version_111-x86_64-linux.tar.gz -O - | tar -xzv binaryen-version_111/bin/wasm-opt --strip-components 2

install:
	$(MAKE) install-wasm-opt

build-tez-kernel:
	cd tez_kernel && cargo build --target wasm32-unknown-unknown --release
	# cp ./tez_kernel/target/wasm32-unknown-unknown/release/tez_kernel.wasm ./bin/tez_kernel.wasm
	./bin/wasm-opt -Os -o ./bin/tez_kernel.wasm ./tez_kernel/target/wasm32-unknown-unknown/release/tez_kernel.wasm

build:
	$(MAKE) build-tez-kernel

test-tez-kernel:
	cd tez_kernel && cargo test -- --nocapture

test:
	$(MAKE) test-tez-kernel

repl:
	./bin/octez-wasm-repl ./bin/tez_kernel.wasm --inputs ./test/input.json