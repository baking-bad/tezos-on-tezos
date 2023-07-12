# Tezos node

This is a facade node for Tezos smart rollup enabling compatibility at the RPC level.

## Supported endpoints

```
/version
/chains/main/chain_id
/chains/main/is_bootstrapped
/chains/main/mempool/pending_operations
/injection/operation
/chains/main/blocks/{block_id}/helpers/forge/operations
/chains/main/blocks/{block_id}/helpers/preapply/operations
/chains/main/blocks/{block_id}/hash
/chains/main/blocks/{block_id}/header
/chains/main/blocks/{block_id}/metadata
/chains/main/blocks/{block_id}/protocols
/chains/main/blocks/{block_id}/live_blocks
/chains/main/blocks/{block_id}
/chains/main/blocks/{block_id}/context/delegates
/chains/main/blocks/{block_id}/context/delegates/{delegate_id}
/chains/main/blocks/{block_id}/context/constants
/chains/main/blocks/{block_id}/context/big_maps/{big_map_id}/{key_hash}
/chains/main/blocks/{block_id}/context/big_maps/{big_map_id}/{key_hash}/normalized
/chains/main/blocks/{block_id}/context/contracts/{contract_id}/manager_key
/chains/main/blocks/{block_id}/context/contracts/{contract_id}/balance
/chains/main/blocks/{block_id}/context/contracts/{contract_id}/counter
/chains/main/blocks/{block_id}/context/contracts/{contract_id}/delegate
/chains/main/blocks/{block_id}/context/contracts/{contract_id}/storage
/chains/main/blocks/{block_id}/context/contracts/{contract_id}/script
/chains/main/blocks/{block_id}/context/contracts/{contract_id}/script/normalized
/chains/main/blocks/{block_id}/context/contracts/{contract_id}/entrypoints
/chains/main/blocks/{block_id}/context/contracts/{contract_id}
/chains/main/blocks/{block_id}/operations/{pass}/{index}
/chains/main/blocks/{block_id}/operations/{pass}
/chains/main/blocks/{block_id}/operations
/chains/main/blocks/{block_id}/operation_hashes/{pass}/{index}
/chains/main/blocks/{block_id}/operation_hashes/{pass}
/chains/main/blocks/{block_id}/operation_hashes
```

## Mockup mode

Run `mock-node` (no arguments) binary to spin up a stateless facade node with Tezos protocol initialized: it can be used to run e2e test scenarios or for other purposes.

## Sequencer

Currently facade node routes injection queries through the rollup node: 1 injected operation — 1 external inbox message.  
In order to increase the throughput and overcome the limitations (max inbox message size, single manager L1 operation per block) we will integrate DAC* (and eventually DAL**) solution.  
In order to decrease latency we will switch to the optimistic sequencer node provided by the Kernel SDK.