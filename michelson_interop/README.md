# Michelson <> Rust interoperability

This crate enables interoperability between Rust and Michelson type systems.  
In particular:
- Exports `MichelsonInterop` trait that specifies type mapping and encoding/decoding functions
- Provides several implementations of this trait for generic collections and for Tezos-specific types

## Supported types and mappings

### Core

- `unit` -> `()`
- `bool` -> `bool`
- `string` -> `String`
- `bytes` -> `Vec<u8>` (there's also `Bytes` alias)

### Numeric

- `int` -> `tezos_core::types::number::Int`
- `nat` -> `tezos_core::types::number::Nat`
- `mutez` -> `tezos_core::types::mutez::Mutez`

There are also several unambiguous aliases for convenience:
- `ibig::IBig` => `int`
- `ibig::UBig` => `nat`
- `u64` -> `mutez` 

### Domain

- `address` -> `tezos_core::types::encoded::Address`
- `chain_id` -> `tezos_core::types::encoded::ChainId`
- `key` -> `tezos_core::types::encoded::Key`
- `key_hash` -> `tezos_core::types::encoded::ImplicitAddress`
- `signature` -> `tezos_core::types::encoded::Signature`
- `ticket ty` -> `Ticket<T>` (alias for `(Address, T, Nat)`)
- `timestamp` -> `i64` (Unix seconds)

### Generic

Sets/Maps are not being sorted when converting to Michelson (might change in the future).

- `option ty` -> `Option<T>`
- `list ty` -> `Vec<T>`
- `set ty` -> `HashSet<T>`
- `map kty vty` -> `HashMap<T>`

There are two convenient macros provided for initializing maps and sets:
- `hashmap! { key1 => val1, key2 => val2 }`
- `hashset! [ elt1, elt2 ]`

### Algebraic

`MichelsonInterop` is implemented for generic tuples with up to 6 fields.  
For custom structs and enums you need to use derive macro:

```rust
#[derive(MichelsonInterop)]
struct {
    name: String,
    age: Nat
}
```

You can use only types that implement `MichelsonInterop` inside structs/enums:

```rust
#[derive(MichelsonInterop)]
enum {
    Default,
    Do()
}
```

See [michelson_derive](../michelson_derive) crate for more details.