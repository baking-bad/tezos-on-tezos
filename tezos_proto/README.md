# Tezos proto

This is a subset of the Tezos economic protocol implemented in Rust.

## Design

Top-level entities:
* `TezosContext` trait — Tezos global context abstraction, implemented for `LayeredStore`
* `SignedOperation` struct — original operation injected to the network
* `ValidatedOperation` enum — holds either a `ValidOperation` or a list of `RpcError`
* `BatchHeader` struct — batches are basically L2 blocks
* `Head` struct — L2 chain state

Basic features:
* Context migrations: see `context::migrations`
* Operation validation: see `validator::operation`
* Operation execution: see `executor::operation`
* Batching operations: see `batcher`

Note the dependency links for `tezos_*` crates (temporary, will be fixed later):
```toml
tezos_core = { git = "https://github.com/baking-bad/tezos-rust-sdk", branch = "develop", package = "tezos-core", default-features = false, features = ["ed25519"] }
tezos_operation = { git = "https://github.com/baking-bad/tezos-rust-sdk", branch = "develop", package = "tezos-operation", default-features = false, features = ["ed25519"] }
tezos_rpc = { git = "https://github.com/baking-bad/tezos-rust-sdk", branch = "develop", package = "tezos-rpc", default-features = false }
tezos_michelson = { git = "https://github.com/baking-bad/tezos-rust-sdk", branch = "develop", package = "tezos-michelson", default-features = false }
```

## Limitations

Current design is intentionally simplified to speed up development while having a minimal necessary functional to operate.
* No gas/storage metering (although it can be incorporated rather easily)
* No money burning
* Non-sequential account counters
* Only 3 manager operations supported: transaction, reveal, origination
* Branch is currently not validated (infinite TTL)
* Several Michelson features are not supported (see `michelson_vm` crate for more details)

### Test coverage

Partially ported test suite from the Tezos repo: https://gitlab.com/tezos/tezos/-/blob/master/tests_python/tests_016/test_contract_onchain_opcodes.py