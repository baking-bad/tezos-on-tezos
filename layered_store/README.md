# Layered Store
This is a generic type providing a transactional storage interface, i.e. you can do multiple read/write operations and then either commit or revert changes.  
Layered store is implemented as a dynamically typed cache (via `Box<dyn Any>`) plus a generic backend implementing `StoreBackend` trait.

## About
This storage abstraction is intended for use in kernel protocol implementations as well as their dependent crates: this way one can enable interoperability/composability while keeping the implementation details within the according crates. This also resolves the "foreign traits / foreign types" problem which often occurs in a codebase split into many crates.

### Store type
The cache value type is restricted by the `StoreType` trait requiring you to implement serialization/deserialization for every stored type. There are builtin implementations for some basic types (`i64`, `[u8; 32]`, to be extended) and also for several Tezos-specific types (enabled by `tezos` feature) provided by `tezos_core` and `tezos_michelson` crates.

### Store backend
There is an ephemeral backend `EphemeralBackend` (and `EphemeralStore` alias for `LayeredStore<EphemeralBackend>`) for testing and stateless modes, and also a generic implementation of a kernel backend (enabled by `kernel` feature) that holds a mutable reference to an instance implementing `Host: SmartRollupCore`.

Kernel backend introduces another transactional layer: data is first stored under the `/tmp` root and then you can either discard changes by calling `clear` or save them with `persist`. A typical workflow is when you use `commit/rollback` for handling individual transactions, and `persist/clear` for batches.

## Usage
When you need a persistent storage in your kernel protocol:
* Abstract all I/O via some "context" trait and use it everywhere in your protocol (`&mut impl YourContext`);
* Implement `LayeredStore<Backend: StoreBackend>` for `YourContext`;
* Implement `StoreType` for all the types you are storing (see implementation guide below);
* Use `EphemeralStore::default()` for testing.

When implementing `StoreType` trait for your type you can use the following rule:
* If it's a not foreign type — implement the trait in your own crate;
* If it's a foreign type and there's no heavy dependencies (like `serde`), and this type is used frequently across many crates, add the implementation to the `layered_store` itself, possibly behind a feature switch if it's not a generic type;
* Otherwise use the "newtype" pattern and implement `StoreType` trait for the wrapped type `WrappedType(pub YourType)`.
