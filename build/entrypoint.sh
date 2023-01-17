#!/bin/sh

set -e

client_dir="/root/.tezos-client"
rollup_dir="/root/.tezos-smart-rollup-node"
endpoint="https://rpc.$NETWORK.teztnets.xyz"
faucet="https://faucet.$NETWORK.teztnets.xyz"

command=$1
shift 1

launch_rollup_node() {
    if [ ! -f "$rollup_dir/config.json" ]; then
        echo "Generating operator config..."
        if [ -z "$ROLLUP_ADDRESS" ]; then
            echo "ROLLUP_ADDRESS is not set"
            exit 1
        fi
        if [ -z "$OPERATOR_ADDRESS" ]; then
            echo "OPERATOR_ADDRESS is not set"
            exit 2
        fi
        octez-smart-rollup-node --base-dir "$client_dir" init operator config for "$ROLLUP_ADDRESS" with operators "$OPERATOR_ADDRESS" --data-dir "$rollup_dir"
    fi
    TEZOS_LOG='* -> info' exec octez-smart-rollup-node --endpoint "$endpoint" -d "$client_dir" run --data-dir "$rollup_dir" --rpc-addr "0.0.0.0"
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
    if [ ! -f "$rollup_dir/kernel.wasm" ]; then
        echo "Kernel not found"
        exit 1
    fi
    kernel="$(xxd -p "$rollup_dir/kernel.wasm" | tr -d '\n')"
    
    octez-client --endpoint "$endpoint" originate smart rollup from "$ORIGINATOR_KEY" of kind wasm_2_0_0 of type bytes with kernel "$kernel" --burn-cap 999 | tee originate.out
    rollup_address=$(cat originate.out | grep -oE "sr1.*")
    if [ -z "$rollup_address" ]; then
        echo "Failed to parse rollup address"
        exit 2
    else
        echo "Originated rollup: $rollup_address"
    fi
    if [ -z "$OPERATOR_ADDRESS" ]; then
        echo "OPERATOR_ADDRESS is not set, using originator address"
        OPERATOR_ADDRESS=$(cat originate.out | grep From | grep -oE "tz.*" | uniq)
    fi
    octez-smart-rollup-node --base-dir "$client_dir" init operator config for "$rollup_address" with operators "$OPERATOR_ADDRESS" --data-dir "$rollup_dir"
}

generate_keypair() {
    octez-client --endpoint "$endpoint" gen keys "operator"
    operator_address=$(octez-client --endpoint "$endpoint" show address "operator" 2>&1 | grep Hash | grep -oE "tz.*")
    echo "Top up the balance for $operator_address on $faucet"
}

populate_inbox() {
    octez-client --endpoint "$endpoint" send smart rollup message "file:$@" from operator
}

case $command in
    rollup-node)
        launch_rollup_node
        ;;
    originate-rollup)
        originate_rollup
        ;;
    generate-keypair)
        generate_keypair
        ;;
    wasm-repl)
        octez-wasm-repl $@
        ;;
    populate-inbox)
        populate_inbox $@
        ;;
    *)
        cat <<EOF
Available commands:

Daemons:
- rollup-node

Commands:
  - originate-rollup
  - generate-keypair
  - wasm-repl [kernel.wasm] --inputs [inputs.json]
  - populate-inbox [messages.json]

EOF
        ;;
esac
