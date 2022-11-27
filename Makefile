.PHONY: bin test

bin:
	docker build -t tezos-bin ./build/octez-wasm-repl
	container_id=$$(docker create tezos-bin)
	docker cp "$$container_id:/octez-wasm-repl" ./bin/octez-wasm-repl
	docker rm "$$container_id"

build:
	cd kernel && cargo build --target wasm32-unknown-unknown
	cp ./kernel/target/wasm32-unknown-unknown/debug/tez_kernel.wasm ./bin/

test:
	cd kernel && cargo test -- --nocapture

repl:
	./bin/octez-wasm-repl ./bin/tez_kernel.wasm --inputs ./test/input.json