# Michelson derive

This crate provides two procedure macros:
- #[derive(MichelsonInterop)] for `struct` and `enum`
- `michelson_tuple!` for tuples

#### Tuples

In order to derive `MichelsonInterop` trait for generic tuples:

```rust
michelson_derive::michelson_tuple!(A, B, C);
```
Currently up to 6 fields, extend if you need more:

- `pair a b` -> `(A, B)`
- `pair a b c` -> `(A, B, C)`
- `pair a b c d` -> `(A, B, C, D)`
- `pair a b c d e` -> `(A, B, C, D, E)`
- `pair a b c d e f` -> `(A, B, C, D, E, F)`

#### Structs

Structs are allowed to have no fields (Unit), single unnamed field, or two and more named/unnamed fields.

- `struct S {}` -> `unit`
- `struct S ()` -> `unit`
- `struct (A)` -> `a`
- `struct (A, B)` -> `pair a b`
- `struct { a: A, b: B }` -> `pair (a %a) (b %b)` (named tuple)

Nested pairs are always expanded to right combs.

#### Enums

Unit, newtype (one and more unnamed fields), and struct variants are supported (two and more named fields).

```rust
enum E {
    Default,
    EntrypointOne(A, B), 
    EntrypointTwo { a: A, b: B }
}
```

Variant names are converted to snake case:

```
(or (unit %default) 
    (or (pair %entrypoint_one a b) 
        (pair %entrypoint_two (a %a) (b %b))))
```

Nested ors are always expanded to right combs.
