use tezos_michelson::michelson::data::Instruction;
use tezos_michelson::micheline::{
    Micheline, primitive_application
};
use tezos_operation::operations::OperationContent;
use tezos_rpc::models::operation::operation_result::lazy_storage_diff::LazyStorageDiff;
use tezos_core::types::{
    encoded::{ImplicitAddress, Address, ContractAddress},
    mutez::Mutez
};

use crate::{
    Result,
    error::InterpreterError,
    vm::stack::Stack,
    constants::*,
    context::Context
};

pub struct TransactionScope {
    pub source: ImplicitAddress,
    pub sender: Address,
    pub amount: Mutez,
    pub entrypoint: String,
    pub parameter: Micheline,
    pub storage: Micheline,
    pub now: i64,
    pub self_address: ContractAddress,
    pub level: i32,
}

pub struct TransactionResult {
    pub storage: Micheline,
    pub internal_operations: Vec<OperationContent>,
    pub lazy_storage_diff: Vec<LazyStorageDiff>
}

impl TransactionScope {
    pub fn default() -> Self {
        Self {
            amount: 0u32.into(),
            level: 0.into(),
            now: 0,
            entrypoint: "default".into(),
            parameter: Micheline::PrimitiveApplication(primitive_application("Unit")),
            storage: Micheline::PrimitiveApplication(primitive_application("Unit")),
            self_address: DEFAULT_ORIGINATED_ADDRESS.try_into().unwrap(),
            sender: DEFAULT_IMPLICIT_ADDRESS.try_into().unwrap(),
            source: DEFAULT_IMPLICIT_ADDRESS.try_into().unwrap(),
        }
    }
}

pub trait Interpreter {
    fn execute(&self, stack: &mut Stack, tx_scope: &TransactionScope, global_ctx: &mut impl Context) -> Result<()>;
}

pub trait PureInterpreter {
    fn execute(&self, stack: &mut Stack) -> Result<()>;
}

pub trait ScopedInterpreter {
    fn execute(&self, stack: &mut Stack, tx_scope: &TransactionScope) -> Result<()>;
}

pub trait ContextIntepreter {
    fn execute(&self, stack: &mut Stack, global_ctx: &mut impl Context) -> Result<()>;
}

impl Interpreter for Instruction {
    fn execute(&self, stack: &mut Stack, tx_scope: &TransactionScope, global_ctx: &mut impl Context) -> Result<()> {
        match self {
            Instruction::Push(instr) => instr.execute(stack),
            Instruction::Drop(instr) => instr.execute(stack),
            Instruction::Dup(instr) => instr.execute(stack),
            Instruction::Swap(instr) => instr.execute(stack),
            Instruction::Dig(instr) => instr.execute(stack),
            Instruction::Dug(instr) => instr.execute(stack),
            Instruction::Rename(_) => Ok(()),
            Instruction::Cast(_) => Ok(()),
            Instruction::FailWith(instr) => instr.execute(stack),
            Instruction::Sequence(seq) => seq.execute(stack, tx_scope, global_ctx),
            Instruction::Dip(instr) => instr.execute(stack, tx_scope, global_ctx),
            Instruction::If(instr) => instr.execute(stack, tx_scope, global_ctx),
            Instruction::IfCons(instr) => instr.execute(stack, tx_scope, global_ctx),
            Instruction::IfLeft(instr) => instr.execute(stack, tx_scope, global_ctx),
            Instruction::IfNone(instr) => instr.execute(stack, tx_scope, global_ctx),
            Instruction::Loop(instr) => instr.execute(stack, tx_scope, global_ctx),
            Instruction::LoopLeft(instr) => instr.execute(stack, tx_scope, global_ctx),
            Instruction::Map(instr) => instr.execute(stack, tx_scope, global_ctx),
            Instruction::Iter(instr) => instr.execute(stack, tx_scope, global_ctx),
            Instruction::Lambda(instr) => instr.execute(stack),
            Instruction::Apply(instr) => instr.execute(stack),
            Instruction::Exec(instr) => instr.execute(stack, tx_scope, global_ctx),
            Instruction::Abs(instr) => instr.execute(stack),
            Instruction::Add(instr) => instr.execute(stack),
            Instruction::Ediv(instr) => instr.execute(stack),
            Instruction::Lsl(instr) => instr.execute(stack),
            Instruction::Lsr(instr) => instr.execute(stack),
            Instruction::Mul(instr) => instr.execute(stack),
            Instruction::Neg(instr) => instr.execute(stack),
            Instruction::Sub(instr) => instr.execute(stack),
            Instruction::Int(instr) => instr.execute(stack),
            Instruction::IsNat(instr) => instr.execute(stack),
            Instruction::Or(instr) => instr.execute(stack),
            Instruction::Xor(instr) => instr.execute(stack),
            Instruction::And(instr) => instr.execute(stack),
            Instruction::Not(instr) => instr.execute(stack),
            Instruction::Compare(instr) => instr.execute(stack),
            Instruction::Eq(instr) => instr.execute(stack),
            Instruction::Neq(instr) => instr.execute(stack),
            Instruction::Gt(instr) => instr.execute(stack),
            Instruction::Ge(instr) => instr.execute(stack),
            Instruction::Lt(instr) => instr.execute(stack),
            Instruction::Le(instr) => instr.execute(stack),
            Instruction::Size(instr) => instr.execute(stack),
            Instruction::Slice(instr) => instr.execute(stack),
            Instruction::Concat(instr) => instr.execute(stack),
            Instruction::Pack(instr) => instr.execute(stack),
            Instruction::Unpack(instr) => instr.execute(stack),
            Instruction::Unit(instr) => instr.execute(stack),
            Instruction::Car(instr) => instr.execute(stack),
            Instruction::Cdr(instr) => instr.execute(stack),
            Instruction::Pair(instr) => instr.execute(stack),
            Instruction::Unpair(instr) => instr.execute(stack),
            Instruction::None(instr) => instr.execute(stack),
            Instruction::Some(instr) => instr.execute(stack),
            Instruction::Left(instr) => instr.execute(stack),
            Instruction::Right(instr) => instr.execute(stack),
            Instruction::Nil(instr) => instr.execute(stack),
            Instruction::Cons(instr) => instr.execute(stack),
            Instruction::EmptySet(instr) => instr.execute(stack),
            Instruction::EmptyMap(instr) => instr.execute(stack),
            Instruction::Mem(instr) => instr.execute(stack, global_ctx),
            Instruction::Get(instr) => instr.execute(stack, global_ctx),
            Instruction::Update(instr) => instr.execute(stack, global_ctx),
            Instruction::GetAndUpdate(instr) => instr.execute(stack, global_ctx),
            _ => Err(InterpreterError::MichelsonInstructionUnsupported { instruction: self.clone() }.into())
        }
    }
}