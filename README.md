# Tezos on Tezos

Optimistic rollup enabled with Tezos VM running on top of Tezos L1.

**IMPORTANT: THIS IS AN EARLY BETA, DO NOT RUN THIS CODE IN THE MAINNET**

## About

The goal of this project is to create PoC of a permissioned application-specific rollup enabled with Tezos VM.  
Aside from the pure research interest there might be long-term advantages of such solution:
* Reduced operational costs (contract automation, oracles)
* Custom MEV-resistant techniques
* Chain-native tokenomics
* Feeless experience
* Contract wallets as first-class citizens
* Potentially smaller operation latency
* Alternative VMs for executing smart contracts (WASM)

## Roadmap

- [x] MVP Tezos-compatible kernel supporting plain transactions and public key reveals
- [x] Installer kernel
- [x] DAC encoding tool
- [x] Docker image with SCORU node, installer, and encoded Tez kernel
- [x] Run TZ rollup in Mondaynet, prepare setup scripts
- [x] Troubleshoot kernel using REPL, get rid of `f64`
- [x] Implement internal batch workflow
- [x] Support origination operation kind
- [x] Implement a minimal viable Michelson interpreter
- [x] Interact with the kernel via inbox and access rollup state via RPC
- [x] Support contract calls and internal transactions
- [x] Tezos RPC facade node
- [x] Deploy a periodic testnet
- [x] Add support to BCD
- [ ] Permanent testnet
- [ ] Add missing Michelson features necessary to onboard first dapps
- [ ] Increase test coverage
- [ ] Spam-prevention mechanism
- [ ] Configurable gas/storage metering
- [ ] Sequencer fees
- [ ] Micheline (de)serialization derive macros
- [ ] WASM smart contracts

## Limitations

Current design is intentionally simplified to speed up development while having a minimal necessary functional to operate.
* No gas/storage metering (although it can be incorporated rather easily)
* No money burning
* Non-sequential account counters
* Only 3 manager operations supported: transaction, reveal, origination
* Branch is currently not validated (infinite TTL)
* BigMaps cannot be copied/removed, but can be moved (Rust-like semantics)
* No temporary BigMap allocations
* Several Michelson features are not supported
* Only wallet/indexer RPC endpoints are exposed

## Installation

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

## Build

To build the kernel and its installer:
```
make build-operator
```

Then you can create a local docker image (depending on target network):
```
make image-operator-daily
make image-operator-monday
make image-operator-mumbai
```

The difference is mainly in Octez binaries shipped together with the kernel.

To build a facade node and its docker image:
```
make build-facade
make image-facade
```

## How to run

Run `make generate-keypair` to initialize a Tezos L1 account, and top it up using https://teztnets.xyz faucet, depending on the network you are going to use.

### Operator

Build kernel and its installer, then originate a new rollup, and run a rollup node.  

For Mondaynet and Dailynet:
1. Check that latest periodic network has correct settings in the Makefile (needs to be updated manually)
2. Depending on the target L1 network run

```
make daily
make monday
make mumbai
```

### Operator + Facade

Once you have both operator and facade images built, you can run them together with compose.

First, create a `local.env` file with two environment variables:
```
ROLLUP_ADDRESS=<sr rollup address from node logs>
OPERATOR_KEY=unencrypted:<edsk private key from .tezos-client folder>
```

Then run docker-compose specifying the image tag:

```
TAG=monday docker-compose up -d
```

## How to test

### Unit & integration tests

Make sure you have nextest installed:
```
cargo install cargo-nextest --locked
```

Run all tests:
```
make nextest
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

Make sure kernel state is updated:
```
> show key /head
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

Known issues with common crates:
- `chrono` => disable `clock` feature

## Credits

* [Tezos SCORU](https://gitlab.com/tezos/tezos) & [Kernel SDK](https://gitlab.com/tezos/kernel) — Nomadic Labs and TriliTech teams
* [Tezos Rust SDK](https://github.com/airgap-it/tezos-rust-sdk) — Papers team
* Thanks to [@emturner](https://github.com/emturner) and [@romarq](https://github.com/romarq) for accepting my PRs :)