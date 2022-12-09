#!/bin/sh

set -e

client_dir="/root/.tezos-client"
rollup_dir="/root/.rollup-node"
monday="2022-12-05"
endpoint="https://rpc.mondaynet-$monday.teztnets.xyz"
faucet="https://faucet.mondaynet-$monday.teztnets.xyz"
command=$1

launch_rollup_node() {
    if [ ! -f "$rollup_dir/config.json" ]; then
        echo "Generating operator config..."
        if [ -z "$ROLLUP_ADDRESS" ]; then
            echo "ROLLUP_ADDRESS is not set"
            exit -1
        fi
        if [ -z "$OPERATOR_ADDRESS" ]; then
            echo "OPERATOR_ADDRESS is not set"
            exit -1
        fi
        octez-sc-rollup-node --base-dir "$client_dir" init operator config for "$ROLLUP_ADDRESS" with operators "$OPERATOR_ADDRESS" --data-dir "$rollup_dir"
    fi
    exec octez-sc-rollup-node --endpoint "$endpoint" -d "$client_dir" run --data-dir "$rollup_dir"
}

originate_rollup() {
    if [ -f "$rollup_dir/config.json" ]; then
        echo "Found existing rollup config"
        exit 0
    fi
    if [ -z "$ORIGINATOR_KEY" ]; then
        echo "ORIGINATOR_KEY is not set, using 'operator'"
        ORIGINATOR_KEY="operator"
    fi
    if [ -f "$rollup_dir/kernel.wasm" ]; then
        echo "Kernel not found"
        exit -1
    fi
    kernel="$(xxd -p "$rollup_dir/kernel.wasm" | tr -d '\n')"
    
    octez-client --endpoint "$endpoint" originate sc rollup from "$ORIGINATOR_KEY" of kind wasm_2_0_0 of type bytes booting with "$kernel" -burn-cap 999 > originate.out
    rollup_address=$(cat originate.out | grep -oE "scr1.*")
    if [ -z "$rollup_address" ]; then
        echo "Failed to parse rollup address"
        exit -1
    else
        echo "Originated rollup: $rollup_address"
    fi
    if [ -z "$OPERATOR_ADDRESS" ]; then
        echo "OPERATOR_ADDRESS is not set, using originator address"
        OPERATOR_ADDRESS=$(cat originate.out | grep From | grep -oE "tz.*" | uniq)
    fi
    octez-sc-rollup-node --base-dir "$client_dir" init operator config for "$rollup_address" with operators "$OPERATOR_ADDRESS" --data-dir "$rollup_dir"
}

generate_keypair() {
    octez-client --endpoint "$endpoint" gen keys "operator"
    operator_address=$(octez-client --endpoint "$endpoint" show address "operator" 2>&1 | grep Hash | grep -oE "tz.*")
    echo "Top up the balance for $operator_address on $faucet"
}

case $command in
    rollup-node)
        launch_rollup_node
        ;;
    originate-rollup)
        originate_rollup
        ;;
    generate-keypair)
        generate_keypair "$@"
        ;;    
    *)
        cat <<EOF
Available commands:

Daemons:
- rollup-node

Commands:
  - originate-rollup
  - generate-keypair
    
EOF
        ;;
esac
