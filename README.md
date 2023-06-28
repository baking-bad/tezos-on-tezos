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

## How to build

### Binaries

#### Kernel

Create wasm file for the payload kernel:

```
make build-kernel PACKAGE=tezos_kernel
```

#### Installer

Convert payload kernel into 4kb pages and create a boot wasm file:

```
make build-installer PACKAGE=tezos_kernel
```

#### Facade node

Create a binary for the facade node:

```
make build-facade
```

### Docker images

Creates local images out of the pre-built artifacts.

### Rollup node

Requires installer kernel and generated pages.  
Note the environment file included in the Makefile, that exposes `OCTEZ_TAG`, `OCTEZ_PROTO`.

```
make image-operator PACKAGE=tezos_kernel
```

### Facade node

```
make image-facade
```

## How to run

Note the environment file included in the Makefile, that exposes target `NETWORK`.

### Operator

Depending on the target package run:

```
make run-tezos-operator
```

You will end up inside the docker container shell.  
Every time you call this target, kernel and docker image will be rebuilt.

#### Generate new keys

For convenience, your local .tezos-client folder is mapped into the container in order to preserve the keys. Upon the first launch you need to create new keypair, in order to do that inside the operator shell:

```
$ operator generate_key
```

#### Check account info

If you already have a key, check it's balance: it should be at least 10k tez to operate a rollup, otherwise top up the balance from the faucet. To get your account address:

```
$ operator account_info
```

#### Originate rollup

```
$ operator deploy_rollup
```

Rollup data is persisted meaning that you can restart the container without data loss. If you try to call this command again it will tell you that there's an existing rollup configuration. Use `--force` flag to remove all data and originate a new one.

#### Run rollup node

```
$ operator run_node
```

Runs rollup node in synchronous mode, with logs being printed to stdout.

## Facade

Run tezos node binary with debug logs enabled:

```
$ make run-facade
```

Every time you call this target tezos node binary will be rebuilt.

## Docker compose

Once you have both operator and facade images built, you can run them together with compose.

First, create a `.env` file with four environment variables:
```
TAG=<operator image tag>
NETWORK=<destination network name>
ROLLUP_ADDRESS=<sr rollup address from node logs>
OPERATOR_KEY=unencrypted:<edsk private key from .tezos-client folder>
```

Then run docker-compose:

```
docker-compose up -d
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
make debug-kernel PACKAGE=tezos_kernel
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

### Unsupported target `wasm32-unknown-unknown`

Known issues:
- `getrandom` (does not compile since version 0.2.10) => use patched version
    ```toml
    [patch.crates-io]
    getrandom = { git = "https://github.com/m-kus/getrandom", branch = "patch/0.2" }
    ```

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