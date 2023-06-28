# Sapling proto

This is the Tezos-adapted implementation of the Sapling protocol in Rust.

## About

The implementation is aimed to be compatible with the Tezos tooling, in particular:
* Binary codec of the Michelson type `sapling_transaction` is used as is
* Storage layout for commitments, ciphertexts, nullifiers, and roots resembles the one used in Tezos

## Limitation

Sapling states are currently not distinguished, assuming there's only one state (as opposed to Tezos where multiple states identified by ID with different memo size can co-live). This will be fixed in the next versions.