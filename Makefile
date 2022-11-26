.PHONY: bin kernel

bin:
	docker build -t tezos-bin ./build/octez-wasm-repl
	container_id=$$(docker create tezos-bin)
	docker cp "$$container_id:/octez-wasm-repl" ./bin/octez-wasm-repl
	docker rm "$$container_id"

kernel:
	cd kernel && cargo build --target wasm32-unknown-unknown
	cp ./kernel/target/wasm32-unknown-unknown/debug/tezos_kernel.wasm ./bin/

repl:
	./bin/octez-wasm-repl ./bin/tezos_kernel.wasm