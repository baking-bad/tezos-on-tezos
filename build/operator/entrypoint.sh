#!/bin/sh

set -e

client_dir="/root/.tezos-client"
rollup_dir="/root/.tezos-smart-rollup-node"
endpoint="https://rpc.$NETWORK.teztnets.xyz"
faucet="https://faucet.$NETWORK.teztnets.xyz"
debug_log_config="file-descriptor-path:///root/logs/kernel_debug.log?name=kernel_debug&chmod=0o644"

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

launch_rollup() {
    import_key()

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

    # if [[ $* == "--debug" ]]; then
    #     log_config=$debug_log_config
    # fi

    TEZOS_LOG='* -> info' TEZOS_EVENTS_CONFIG=$log_config exec octez-smart-rollup-node --endpoint "$endpoint" -d "$client_dir" run --data-dir "$rollup_dir" --rpc-addr "0.0.0.0"
}

originate_rollup() {
    import_key()

    if [ -f "$rollup_dir/config.json" ]; then
        echo "Found existing rollup config"
        exit 0
    fi
   
    if [ ! -f "/root/kernel.wasm" ]; then
        echo "Kernel not found"
        exit 1
    fi
    kernel="$(xxd -p "/root/kernel.wasm" | tr -d '\n')"
    
    octez-client --endpoint "$endpoint" originate smart rollup from operator of kind wasm_2_0_0 of type bytes with kernel "$kernel" --burn-cap 999 | tee originate.out
    rollup_address=$(cat originate.out | grep -oE "sr1.*")
    if [ -z "$rollup_address" ]; then
        echo "Failed to parse rollup address"
        exit 2
    else
        echo "Originated rollup: $rollup_address"
    fi

    operator_address=$(octez-client --endpoint "$endpoint" show address "operator" 2>&1 | grep Hash | grep -oE "tz.*")
    octez-smart-rollup-node --base-dir "$client_dir" init operator config for "$rollup_address" with operators "$operator_address" --data-dir "$rollup_dir"
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
        launch_rollup
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
- rollup-node --debug

Commands:
  - originate-rollup
  - generate-keypair
  - wasm-repl [kernel.wasm] --inputs [inputs.json]
  - populate-inbox [messages.json]

EOF
        ;;
esac
