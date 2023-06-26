# Tezos on Tezos

Optimistic rollup enabled with Tezos VM running on top of Tezos L1.

**IMPORTANT: NOT STABLE YET, DO NOT RUN THIS CODE IN PRODUCTION**

## About

The goal of this project is to create permissioned application-specific Tezos-compatible rollup that has:
* Reduced operational costs (contract automation, oracles)
* Custom MEV-resistant solution
* Chain-native tokenomics
* Feeless experience
* Contract wallets as first-class citizens (account abstraction)

## How to play

`Rollupnet` is a public deployment of the smart rollup operator + Tezos-compatible facade node, it is used mostly for testing and demonstrating purposes.  

### RPC

Public endpoint:
* https://rollupnet.zaun.baking-bad.org/chains/main/blocks/head

You need to add custom network to your wallet, if you want to interact with the `rollupnet`. Check out this tutorial on how to add custom RPC provider to Temple wallet: https://www.youtube.com/watch?v=VzeSFdna8Vk

### Keys

Import one of the following bootstrap keys:
* `edsk3gUfUPyBSfrS9CCgmCiQsTCHGkviBDusMxDJstFtojtc1zcpsh`
* `edsk39qAm1fiMjgmPkw1EgQYkMzkJezLNewd7PLNHTkr6w9XA2zdfo`
* `edsk4ArLQgBTLWG5FJmnGnT689VKoqhXwmDPBuGx3z4cvwU9MmrPZZ`
* `edsk2uqQB9AY4FvioK2YMdfmyMrer5R8mGFyuaLLFfSRo8EoyNdht3`
* `edsk4QLrcijEffxV31gGdN2HU7UpyJjA8drFoNcmnB28n89YjPNRFm`

### BCD

We have a dedicated instance of Better Call Dev explorer for periodic test networks including `rollupnet`:
* https://teztnets.better-call.dev/

You can use it to deploy and interact with smart contracts using web interface.

### Limitations

* With the current limit (4096 bytes) for inbox messages you won't be able to deploy large smart contracts
* Some Michelson features are not yet supported, so you might not able to deploy contracts containing particular opcodes

### Feedback

Your feedback is extremely valuable, and we also expect lots of bugs at early stage, so please contact us if anything works not as expected:
* [Discord](https://discord.com/invite/RcPGSdcVSx) server
* [Telegram](https://t.me/baking_bad_chat) chat
* [Slack](https://tezos-dev.slack.com/archives/CV5NX7F2L) channel

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

Then you can create a local docker image, depending on target network:
- `make image-operator-monday`
- `make image-operator-mumbai`

Other options are not in the Makefile, but you can add them yourself, the difference is mainly in Octez binaries shipped together with the kernel.

To build a facade node and its docker image:
```
make build-facade
make image-facade
```

## How to run

Run `make generate-keypair` to initialize a Tezos L1 account, and top it up using https://teztnets.xyz faucet, depending on the network you are going to use.

### Operator

Build kernel and its installer, then originate a new rollup, and run a rollup node.  
Depending on the target L1 network run one of:
- `make operator-monday`
- `make operator-mumbai`

Other options are not in the Makefile, but you can add them yourself based on the existing ones.

Note that every time you run this target a new rollup will be deployed, so make sure you have enough funds for a 10k bond. Use [faucet](https://teztnets.xyz/) to top up your account.

In order to just run operator with an existing rollup:

```
make rollup-node TAG=monday
```

### Facade

The following target will build the facade node and run it with default arguments:

```
make facade
```

### Docker compose

Once you have both operator and facade images built, you can run them together with compose.

First, create a `local.env` file with two environment variables:
```
ROLLUP_ADDRESS=<sr rollup address from node logs>
OPERATOR_KEY=unencrypted:<edsk private key from .tezos-client folder>
```

Then run docker-compose specifying the image tag, e.g.:

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