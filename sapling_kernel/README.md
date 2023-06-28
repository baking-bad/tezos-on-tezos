# Sapling kernel

Smart rollup kernel implementing Tezos-adapted Sapling protocol for privacy preserving transactions.

## About

The behavior of this kernel is analogues to a simple sapling pool on Tezos.  

## How to send transactions

Prepare a Sapling transaction using one of the existing tool:
* Taquito https://tezostaquito.io/docs/sapling/#how-to-prepare-a-sapling-transaction
* tezos-client https://tezos.gitlab.io/alpha/sapling.html#sandbox-tutorial

Send resulting payload as an external inbox message:
https://tezos.gitlab.io/alpha/smart_rollups.html#sending-an-external-inbox-message

## How to get ciphertexts

Assuming your rollup node is running at `http://localhost:8932`, access the storage via the following requests:
* Get ciphertext at position 1 `http://localhost:8932/global/block/head/durable/wasm_2_0_0/value?key=/ciphertexts/1`
* Get total ciphertexts count `http://localhost:8932/global/block/head/durable/wasm_2_0_0/value?key=/head`