#!/bin/sh

# SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
#
# SPDX-License-Identifier: MIT

set -e

client_dir="/root/.tezos-client"
rollup_dir="/root/.tezos-smart-rollup-node"
endpoint=$NODE_URI
faucet="https://faucet.$NETWORK.teztnets.xyz"

if [ -z "$NODE_URI" ]; then
    if [ -z "$NETWORK" ]; then
        echo "NETWORK is not set"
        exit 1
    fi
    endpoint="https://rpc.$NETWORK.teztnets.xyz"
fi

command=$1
shift 1

import_key() {
    if [ ! -f "$client_dir/secret_keys" ]; then
        echo "Importing operator key..."
        if [ -z "$OPERATOR_KEY" ]; then
            echo "OPERATOR_KEY is not set"
            exit 2
        fi
        octez-client --endpoint "$endpoint" import secret key operator "$OPERATOR_KEY"
    fi
}

run_node() {
    import_key

    if [ ! -f "$rollup_dir/config.json" ]; then
        echo "Generating operator config..."
        if [ -z "$ROLLUP_ADDRESS" ]; then
            echo "ROLLUP_ADDRESS is not set"
            exit 1
        fi
        mkdir $rollup_dir || true
        operator_address=$(octez-client --endpoint "$endpoint" show address "operator" 2>&1 | grep Hash | grep -oE "tz.*")
        octez-smart-rollup-node --base-dir "$client_dir" init operator config for "$ROLLUP_ADDRESS" with operators "$operator_address" --data-dir "$rollup_dir"
    fi

    if [ ! -d "$rollup_dir/wasm_2_0_0" ]; then
        echo "Initializing metadata folder..."
        cp -R /root/wasm_2_0_0 "$rollup_dir/wasm_2_0_0"
    fi

    # Write logs to a file: "file-descriptor-path:///kernel_debug.log?name=kernel_debug&chmod=0o644"
    TEZOS_LOG='* -> info' TEZOS_EVENTS_CONFIG=$LOG_CONFIG exec octez-smart-rollup-node --endpoint "$endpoint" -d "$client_dir" run --data-dir "$rollup_dir" --rpc-addr "0.0.0.0"
}

deploy_rollup() {
    import_key

    if [ -f "$rollup_dir/config.json" ]; then
        echo "Found existing rollup config"
        if [ "$1" == "--force" ]; then
            echo "Overriding with new kernel"
            rm -rf "$rollup_dir/*"
            octez-client --endpoint "$endpoint" forget all smart rollups --force
        else
            exit 0
        fi
    fi
   
    if [ ! -f "/root/kernel.wasm" ]; then
        echo "Kernel not found"
        exit 1
    fi
    kernel="$(xxd -p "/root/kernel.wasm" | tr -d '\n')"
    
    octez-client --endpoint "$endpoint" originate smart rollup "rollup" from operator of kind wasm_2_0_0 of type bytes with kernel "$kernel" --burn-cap 999 --force | tee originate.out
    operator_address=$(octez-client --endpoint "$endpoint" show address "operator" 2>&1 | grep Hash | grep -oE "tz.*")
    octez-smart-rollup-node --base-dir "$client_dir" init operator config for "rollup" with operators "$operator_address" --data-dir "$rollup_dir"
}

generate_key() {
    octez-client --endpoint "$endpoint" gen keys "operator"
    operator_address=$(octez-client --endpoint "$endpoint" show address "operator" 2>&1 | grep Hash | grep -oE "tz.*")
    echo "Top up the balance for $operator_address on $faucet"
}

account_info() {
    octez-client --endpoint "$endpoint" show address "operator"
    octez-client --endpoint "$endpoint" get balance for "operator"
    echo "Top up the balance on $faucet"
}

send_message() {
    octez-client --endpoint "$endpoint" send smart rollup message hex:"[\"$1\"]" from operator
}

case $command in
    run_node)
        run_node
        ;;
    deploy_rollup)
        deploy_rollup $@
        ;;
    generate_key)
        generate_key
        ;;
    account_info)
        account_info
        ;;
    send_message)
        send_message $@
        ;;
    *)
        cat <<EOF
Available commands:

Daemons:
- run_node

Commands:
  - account_info
  - generate_key
  - deploy_rollup --force
  - send_message [hex string]

EOF
        ;;
esac
