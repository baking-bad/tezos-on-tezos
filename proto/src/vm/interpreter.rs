use tezos_michelson::michelson::data::{Sequence, Instruction};

use crate::{
    Result,
    Error,
    vm::context::ExecutionContext,
    vm::stack::Stack,
    vm::types::StackItem,
    context::Context
};

pub trait Interpreter {
    fn execute(self, global_ctx: &mut impl Context, stack: &mut Stack, exec_ctx: &ExecutionContext) -> Result<()>;
}

impl Interpreter for Instruction {
    fn execute(self, global_ctx: &mut impl Context, stack: &mut Stack, exec_ctx: &ExecutionContext) -> Result<()> {
        match self {
            Instruction::Push(i) => {
                stack.push(StackItem::from_data(*i.value, &i.r#type)?);
                Ok(())
            },
            _ => Err(Error::MichelsonInstructionUnsupported { instruction: self.clone() })
        }
    }
}