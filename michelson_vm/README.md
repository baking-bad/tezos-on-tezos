# Michelson VM

This is a Michelson interpreter written in Rust.

## About
The purpose of this crate is to provide a standalone, lightweight, and WASM-friendly implementation of the Michelson VM. The applications can vary including educational projects, developer tools, but the initial goal is to use it in a Tezos L2 solution.

### Roadmap

- [ ] Views
- [ ] Events
- [ ] Tickets
- [ ] Missing hash functions (`SHA3`, `KECCAK`)
- [ ] Internal originations
- [ ] Sapling
- [ ] Global constants?

### Known differences
Although the intention is to be as close to the reference implementation as possible, there are still edge cases where Rust-based interpreter behaves differently. In particular:
* Sets and Maps do not require input values to be sorted, they will be sorted internally if necessary
* BigMaps cannot be copied or removed, but can be transferred between contracts (Rust-style move semantics); basically move = copy + remove, another implication is that there is no such thing as temporary BigMap (with negative ID)

### Test coverage

Currently two external test suites are run against the VM implementation:
* TZT by Runtime Verification, source: https://gitlab.com/tezos/tzt-reference-test-suite
* Python E2E tests (were replaced with Tezt in the Tezos codebase), saved copy: https://github.com/baking-bad/pytezos/blob/master/tests/unit_tests/test_michelson/test_repl/test_opcodes.py

Both suites were converted to an intermediary Micheline representation:
* TZT test cases [vm/tests/data/tzt](https://github.com/baking-bad/tezos-on-tezos/tree/master/vm/tests/data/tzt)
* E2E test cases [vm/tests/data/e2e](https://github.com/baking-bad/tezos-on-tezos/tree/master/vm/tests/data/e2e)
* Contract scripts (required for E2E tests) [vm/tests/data/scripts](https://github.com/baking-bad/tezos-on-tezos/tree/master/vm/tests/data/scripts)

## Usage

Michelson is a stack-based language, thus in order to run Michelson code you will need a stack:

```rust
use michelson_vm::Stack;

let mut stack = Stack::new();
```

Depending on the instruction kind, you might need some additional information:
* Pure instructions — require only stack
* Scoped instructions — additionally require immutable operation context (basically parent operation content + some contract-related data)
* Context instructions — can (potentially) mutate global state, e.g. allocate/update/query BigMaps, fetch contract types (entrypoints)

Operation scope is a struct you need to fill:
```rust
pub struct michelson_vm::OperationScope {
    pub chain_id: ChainId,
    pub source: ImplicitAddress,
    pub sender: Address,
    pub amount: Mutez,
    pub balance: Mutez,  // contract balance before the execution
    pub parameters: Option<(String, Micheline)>,  // entrypoint + value
    pub storage: Micheline,  // storage before the execution (or initial storage in case of origination)
    pub now: i64,
    pub self_address: ContractAddress,  // destination in case of transaction, originated_contract in case of origination
    pub self_type: Micheline,  // parameter type
    pub level: i32,
}
```

InterpreterContext is a public trait you need to implement:
```rust
pub trait michelson_vm::InterpreterContext {
    fn get_contract_type(&self, address: &ContractAddress) -> Result<Option<Micheline>>;
    fn set_contract_type(&mut self, address: ContractAddress, value: Micheline) -> Result<()>;
    fn allocate_big_map(&mut self, owner: ContractAddress) -> Result<i64>;
    fn get_big_map_owner(&self, ptr: i64) -> Result<ContractAddress>;
    fn has_big_map_value(&self, ptr: i64, key_hash: &ScriptExprHash) -> Result<bool>;
    fn get_big_map_value(&self, ptr: i64, key_hash: &ScriptExprHash) -> Result<Option<Micheline>>;
    fn set_big_map_value(
        &mut self,
        ptr: i64,
        key_hash: ScriptExprHash,
        value: Option<Micheline>,
    ) -> Result<Option<Micheline>>;
}
```

### REPL

Once you have stack, scope and context initialized, you can run some code.  
This crate does not support parsing Michelson yet, so Micheline is assumed to be the raw representation for both types and data (including instructions).  

A typical workflow is when you have a Micheline JSON file that you want to execute:
```rust
use tezos_michelson::micheline::Micheline;
use tezos_michelson::michelson::data::Instruction;

let micheline: Micheline = serde_json_wasm::from_slice(data.as_slice())?;
let code: Instruction = micheline.try_into()?;
```

For all instructions supported a special `Interpreter` trait is implemented so that you can run it:
```rust
use michelson_vm::interpreter::Interpreter;

code.execute(&mut stack, &scope, &mut context)?;
```

### Michelson script

Another case is when you have a fully-fledged Michelson script with parameter/storage/code sections and a standard input including entrypoint name, parameter and storage values.  

Michelson script is also constructed out of a Micheline expression:
```rust
use michelson_vm::MichelsonScript;

let micheline: Micheline = serde_json_wasm::from_slice(data.as_slice())?;
let script = MichelsonScript::try_from(micheline)?;
```

Once you have operation scope filled and context initialized, you can execute the script:
```rust
let result = script.execute(&scope, &mut context)?;
```

In the result you will get a structure containing new storage, BigMap diffs, and a list of internal operations:
```rust
pub struct michelson_vm::ScriptReturn {
    pub storage: Micheline,
    pub operations: Vec<InternalContent>,
    pub big_map_diff: Vec<BigMapDiff>,
}
```

### Tracing

It can be hard to spot a typechecking/parsing/runtime error when executing Michelson code, that's when tracer comes in handy!  
In order to enable traces add `trace` feature to the crate import in your Cargo file:
```toml
michelson_vm = { git = "https://github.com/baking-bad/tezos-on-tezos", features = ["trace"] }
```

Or if you troubleshoot this crate, you can run any test with trace feature enabled and `RUST_LIB_BACKTRACE` flag set:
```sh
RUST_LIB_BACKTRACE=1 cargo test --jobs 1 --no-fail-fast --test e2e --features trace -- --nocapture --test-threads=1 e2e_map_iter_00.json
```

You will see a beautiful output:

![trace](https://i.imgur.com/EGew4G1.png?)

Also note, that `michelson_vm::Error` has a special method `print` that will write to stdout all the error details plus backtrace if it exists.