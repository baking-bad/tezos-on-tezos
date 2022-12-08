#!/usr/bin/env bash

set -euo pipefail

kernel="$(xxd -p /rollup/kernel.wasm | tr -d '\n')"
