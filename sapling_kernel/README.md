# Sapling kernel

Smart rollup kernel implementing Tezos-adapted Sapling protocol for privacy preserving transactions.

## About

The behavior of this kernel is analogues to a simple sapling pool on Tezos.  
For demo purposes anti replay string is set to constant `KT1PwYL1B8hagFeCcByAcsN3KTQHmJFfDwnjNetXnHfVqm9iesp` — this is the address of a sapling pool contract in Ghostnet https://ghostnet.tzkt.io/KT1PwYL1B8hagFeCcByAcsN3KTQHmJFfDwnj

It can be used to validate kernel behavior by replaying all sapling transactions (basically sending them as external messages to the rollup).

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


## Tutorial

This is a modified version of https://tezos.gitlab.io/alpha/sapling.html#sandbox-tutorial that uses existing contract in the Ghostnet.

```sh
# generate new key
octez-client gen keys bootstrap1
octez-client show address bootstrap1

# top up its balance using https://faucet.ghostnet.teztnets.xyz/
octez-client get balance for bootstrap1

# generate two shielded keys for Alice and Bob and use them for the shielded-tez contract
# the memo size has to be indicated
octez-client sapling gen key alice
octez-client sapling use key alice for contract KT1PwYL1B8hagFeCcByAcsN3KTQHmJFfDwnj --memo-size 8
octez-client sapling gen key bob
octez-client sapling use key bob for contract KT1PwYL1B8hagFeCcByAcsN3KTQHmJFfDwnj --memo-size 8

# generate an address for Alice to receive shielded tokens.
octez-client sapling gen address alice
zet1AliceXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX # Alice's address

# shield 10 tez from bootstrap1 to alice
octez-client sapling shield 10 from bootstrap1 to zet1AliceXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX using KT1PwYL1B8hagFeCcByAcsN3KTQHmJFfDwnj --burn-cap 2

# generate an address for Bob to receive shielded tokens.
octez-client sapling gen address bob
zet1BobXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX # Bob's address

# forge a shielded transaction from alice to bob that is saved to a file
octez-client sapling forge transaction 10 from alice to zet1BobXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX using KT1PwYL1B8hagFeCcByAcsN3KTQHmJFfDwnj

# submit the shielded transaction from any transparent account
octez-client sapling submit sapling_transaction from bootstrap1 using KT1PwYL1B8hagFeCcByAcsN3KTQHmJFfDwnj --burn-cap 1

# unshield from bob to any transparent account
octez-client sapling unshield 10 from bob to bootstrap1 using KT1PwYL1B8hagFeCcByAcsN3KTQHmJFfDwnj --burn-cap 1
```

Now let's send all transaction payloads to the rollup and check that its state is in sync with the sapling state of our contract in Ghostnet.

The easiest way to do that is to open the contract page on [TzKT](https://ghostnet.tzkt.io/KT1PwYL1B8hagFeCcByAcsN3KTQHmJFfDwnj), open raw payload for every operation in the list, and copy sapling transaction bytes (in hex).

Next, for each sapling transaction we need to send an external message to our rollup:

```sh
# In the operator shell
operator send_message %TX_HEX%
```

Now, let's validate the kernel state:
1. General info (number of commitments, nullifiers, roots) -> see `SaplingHead` struct layout  
    https://rpc.tzkt.io/ghostnet/chains/main/blocks/head/context/raw/json/sapling/index/6055/
    http://localhost:8932/global/block/head/durable/wasm_2_0_0/value?key=/head
2. Ciphertexts -> see `Ciphertext` struct layout  
    https://rpc.tzkt.io/ghostnet/chains/main/blocks/head/context/raw/json/sapling/index/6055/ciphertexts/0`
    http://localhost:8932/global/block/head/durable/wasm_2_0_0/value?key=/ciphertexts/0
3. Commitments  
    https://rpc.tzkt.io/ghostnet/chains/main/blocks/head/context/raw/json/sapling/index/6055/commitments/4294967296`
    http://localhost:8932/global/block/head/durable/wasm_2_0_0/value?key=/commitments/4294967296
4. Nullifiers  
    https://rpc.tzkt.io/ghostnet/chains/main/blocks/head/context/raw/json/sapling/index/6055/nullifiers_ordered/0`
    http://localhost:8932/global/block/head/durable/wasm_2_0_0/value?key=/nullifiers_ordered/0
5. Roots  
    https://rpc.tzkt.io/ghostnet/chains/main/blocks/head/context/raw/json/sapling/index/6055/roots/1
    http://localhost:8932/global/block/head/durable/wasm_2_0_0/value?key=/roots/1