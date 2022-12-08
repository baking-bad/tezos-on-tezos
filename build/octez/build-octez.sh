#! /usr/bin/env bash

set -euo pipefail

binaries=("octez-wasm-repl" "octez-sc-rollup-node" "octez-client-alpha")

DOCKER_BUILDKIT=1 docker build -t "ghcr.io/baking-bad/octez:$OCTEZ_VERSION" -f "build/octez/Dockerfile" --build-arg OCTEZ_VERSION="$OCTEZ_VERSION" .
container_id="$(docker create alpine-tezos)"
for b in "${binaries[@]}"; do
    docker cp "$container_id:/usr/bin/$b" "./bin/$b"
done
docker rm -v "$container_id"