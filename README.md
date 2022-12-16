# Tezos on Tezos

Optimistic rollup enabled with Tezos VM running on top of Tezos L1.

**IMPORTANT: THIS IS A RESEARCH PROJECT, DO NOT RUN THIS CODE IN THE MAINNET**

## About

The goal of this project is to create PoC of a permissioned application-specific rollup enabled with Tezos VM.  
Aside from the pure research interest there might be long-term advantages of such solution:
* Reduced operational costs (contract automation, oracles)
* Custom MEV-resistant techniques
* Chain-native tokenomics
* Potentially smaller operation latency

If this project shows good results, we will consider to relocate or partially implement our [DeFi products](https://bakingbad.dev/) in an app-specific rollup.

Also in the scope of our developing tools and indexing stack we want to better understand:
* Which features should we add to [BCD](https://better-call.dev) for SCORU devs
* How to run e2e rollup tests using [Pytezos](https://pytezos.org)
* How to index SCORU (and EVM rollup in particular) for enabling [DipDup](https://dipdup.io) and [TzKT](https://tzkt.io) with the rollup chain data

## Roadmap

- [x] MVP Tezos-compatible kernel supporting plain transactions and public key reveals
- [x] Genesis kernel that installs Tez kernel and initializes seed accounts
- [x] DAC encoding tool
- [x] Docker image with SCORU node, installer, and encoded Tez kernel
- [x] Run TZ rollup in Mondaynet, prepare setup scripts
- [x] Troubleshoot kernel using REPL, get rid of `f64`
- [ ] Interact with the kernel via inbox and access rollup state via RPC
- [ ] Tezos RPC facade node with wallet sufficient endpoint set
- [ ] Add indexer-sufficient endpoints
- [ ] Support origination operation kind
- [ ] Implement a minimal subset of Michelson opcodes
- [ ] Support big maps
- [ ] Support internal transactions
- [ ] Rollup node RPC plugin

## Limitations

Current design is intentionally simplified to speed up development while having a minimal necessary functional to operate.

* Fix-priced operations, just general gas/storage limits
* Non-sequential account counters
* Only 3 manager operations supported: transaction, reveal, origination
* (To be continued)

Non-supported Michelson features (at least in the first iteration):
* Views
* Events
* Big map runtime allocation & copy/removal
* Sapling
* Tickets
* Internal originations

## How to run

Install Rust toolchain:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Add Wasm32 target:
```
rustup target add wasm32-unknown-unknown
```

Install build dependencies:
```
make install
```

Build kernel and installer:
```
make build
```

Create operator image:
```
make image
```

Generate key pair and top up your balance:
```
make generate-keypair
```

Originate rollup in Mondaynet:
```
make originate-rollup
```

Now you can run the operator node in daemon mode
```
make rollup-node
```

You can also run container in interactive mode:
```
make operator-shell
```

## How to test

### Unit tests

Run all unit tests:
```
make test
```

### Wasm REPL

Prepare inputs using notebooks (make sure you have Python installed):
```
jupyter notebook
# navigate to scripts folder
```

Build kernel in debug mode, create docker image, and run REPL:
```
make debug
```

Populate rollup inbox:
```
> load inputs
```

Run kernel:
```
> step inbox
```

Make sure operation receipt is saved:
```
> show key /context/blocks/0/operations/1
```

## Troubleshooting

### `float instructions are forbidden`

SCORU host does not support operations with floating point thus one need to make sure none of the dependencies introduces them.  
Don't forget to use `wasm-strip` to eliminate dead code.  

Known issues with common crates:
- `serde_json` => use `serde-json-wasm` instead
- `num-bigint` (floats used in `to_radix_str` and `from_radix_str`) => use `ibig` instead

In order to trace back float usage, first build kernel with debug info and generate `.wat` file:
```
make wat
```

Search for `f64 ` and `f32 ` substrings and unwind calls up to the crate level.

### `unknown import "__wbindgen_placeholder__"`

Some of your dependencies use `wasm-bindgen` and mistakenly treat `wasm32-unknown-unknown` target as browser env.  
Make sure you have disabled features that do so, or replace such dependencies.

## Credits

* [Tezos SCORU](https://gitlab.com/tezos/tezos) & [Kernel SDK](https://gitlab.com/tezos/kernel) — Nomadic Labs and TriliTech teams
* [Tezos Rust SDK](https://github.com/airgap-it/tezos-rust-sdk) — Papers team
* Thanks to [@emturner](https://github.com/emturner) and [@romarq](https://github.com/romarq) for accepting my PRs :)