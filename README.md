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
- [ ] Genesis kernel that installs Tez kernel and initializes seed accounts
- [x] DAC encoding tool
- [ ] Docker image with SCORU node, installer, and encoded Tez kernel
- [ ] Run Tez rollup in Mondaynet
- [ ] Pytezos bindings
- [ ] E2E integration tests
- [ ] Tezos RPC facade node with indexer-sufficient endpoint set + inject
- [ ] Operation simulation (`run_code` helper)
- [ ] Support originations
- [ ] Support a minimal necessary subset of Michelson opcodes
- [ ] Rollup operation batcher (using reveal channel)

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

## Credits

* [Tezos SCORU](https://gitlab.com/tezos/tezos) & [Kernel SDK](https://gitlab.com/tezos/kernel) — Nomadic Labs and TriliTech teams
* [Tezos Rust SDK](https://github.com/airgap-it/tezos-rust-sdk) — Papers team
* Thanks to [@emturner](https://github.com/emturner) and [@romarq](https://github.com/romarq) for accepting my PRs :)