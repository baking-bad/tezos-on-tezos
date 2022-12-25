pub mod typecheck;
pub mod context;
pub mod stack;

use tezos_michelson::{
    micheline::Micheline
};

use crate::{
    context::Context as GlobalContext,
    vm::context::ExecutionContext,
    vm::stack::{Stack, StackItem},
    Result
};

pub fn interpret(
    global_context: &mut impl GlobalContext, 
    exec_context: &mut ExecutionContext,
    parameter: Micheline,
    storage: Micheline,
    code: Micheline
) -> Result<()> {
    let mut stack = Stack::new();

    

    Ok(())
}