#! /usr/bin/env bash

set -euo pipefail

binaries=("octez-wasm-repl" "octez-sc-rollup-node" "octez-client" "wasm-opt" "wasm-strip")
container_id="$(docker create ghcr.io/baking-bad/scoru-kit)"
for b in "${binaries[@]}"; do
    docker cp "$container_id:/usr/bin/$b" "./bin/$b"
done
docker rm -v "$container_id"