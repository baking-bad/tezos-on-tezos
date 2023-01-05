use tezos_michelson::michelson::data::Instruction;
use tezos_michelson::micheline::Micheline;
use tezos_operation::operations::OperationContent;
use tezos_rpc::models::operation::operation_result::lazy_storage_diff::LazyStorageDiff;
use tezos_core::types::{
    encoded::{ImplicitAddress, Address, ContractAddress, ScriptExprHash, ChainId},
    mutez::Mutez
};

use crate::{
    Result,
    Error,
    stack::Stack,
    types::{StackItem, BigMapItem},
    trace_enter,
    trace_exit
};

pub struct TransactionScope {
    pub chain_id: ChainId,
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

pub trait TransactionContext {
    fn get_balance(&self, address: &Address) -> Result<Option<Mutez>>;
    fn get_contract_type(&self, address: &ContractAddress) -> Result<Option<Micheline>>;
    fn allocate_big_map(&mut self, owner: ContractAddress) -> Result<i64>;
    fn move_big_map(&mut self, ptr: i64, owner: ContractAddress) -> Result<()>;
    fn has_big_map_value(&self, ptr: i64, key_hash: &ScriptExprHash) -> Result<bool>;
    fn get_big_map_value(&self, ptr: i64, key_hash: &ScriptExprHash) -> Result<Option<Micheline>>;
    fn set_big_map_value(&mut self, ptr: i64, key_hash: ScriptExprHash, value: Option<Micheline>) -> Result<Option<Micheline>>;
}

pub struct TransactionResult {
    pub storage: Micheline,
    pub internal_operations: Vec<OperationContent>,
    pub lazy_storage_diff: Vec<LazyStorageDiff>
}

pub trait Interpreter {
    fn execute(&self, stack: &mut Stack, scope: &TransactionScope, context: &mut impl TransactionContext) -> Result<()>;
}

pub trait PureInterpreter {
    fn execute(&self, stack: &mut Stack) -> Result<()>;
}

pub trait ScopedInterpreter {
    fn execute(&self, stack: &mut Stack, scope: &TransactionScope) -> Result<()>;
}

pub trait ContextInterpreter {
    fn execute(&self, stack: &mut Stack, context: &mut impl TransactionContext) -> Result<()>;
}

impl Interpreter for Instruction {
    fn execute(&self, stack: &mut Stack, scope: &TransactionScope, context: &mut impl TransactionContext) -> Result<()> {
        trace_enter!(self);
        let res = match self {
            Instruction::Sequence(seq) => return seq.execute(stack, scope, context),
            Instruction::Push(instr) => instr.execute(stack),
            Instruction::Drop(instr) => instr.execute(stack),
            Instruction::Dup(instr) => instr.execute(stack),
            Instruction::Swap(instr) => instr.execute(stack),
            Instruction::Dig(instr) => instr.execute(stack),
            Instruction::Dug(instr) => instr.execute(stack),
            Instruction::Rename(_) => Ok(()),
            Instruction::Cast(_) => Ok(()),
            Instruction::FailWith(instr) => instr.execute(stack),
            Instruction::Dip(instr) => instr.execute(stack, scope, context),
            Instruction::If(instr) => return instr.execute(stack, scope, context),
            Instruction::IfCons(instr) => return instr.execute(stack, scope, context),
            Instruction::IfLeft(instr) => return instr.execute(stack, scope, context),
            Instruction::IfNone(instr) => return instr.execute(stack, scope, context),
            Instruction::Loop(instr) => instr.execute(stack, scope, context),
            Instruction::LoopLeft(instr) => instr.execute(stack, scope, context),
            Instruction::Map(instr) => instr.execute(stack, scope, context),
            Instruction::Iter(instr) => instr.execute(stack, scope, context),
            Instruction::Lambda(instr) => instr.execute(stack),
            Instruction::Apply(instr) => instr.execute(stack),
            Instruction::Exec(instr) => instr.execute(stack, scope, context),
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
            Instruction::Mem(instr) => instr.execute(stack, context),
            Instruction::Get(instr) => instr.execute(stack, context),
            Instruction::Update(instr) => instr.execute(stack, scope, context),
            Instruction::GetAndUpdate(instr) => instr.execute(stack, scope, context),
            Instruction::Amount(instr) => instr.execute(stack, scope),
            Instruction::ChainId(instr) => instr.execute(stack, scope),
            Instruction::Sender(instr) => instr.execute(stack, scope),
            Instruction::Source(instr) => instr.execute(stack, scope),
            Instruction::Now(instr) => instr.execute(stack, scope),
            Instruction::Level(instr) => instr.execute(stack, scope),
            Instruction::SelfAddress(instr) => instr.execute(stack, scope),
            Instruction::Balance(instr) => instr.execute(stack, scope, context),
            Instruction::Address(instr) => instr.execute(stack),
            Instruction::Contract(instr) => instr.execute(stack, context),
            Instruction::Self_(instr) => instr.execute(stack, scope, context),
            Instruction::ImplicitAccount(instr) => instr.execute(stack, context),
            Instruction::EmptyBigMap(instr) => instr.execute(stack, scope, context),
            _ => Err(Error::MichelsonInstructionUnsupported { instruction: self.clone() }.into())
        };
        trace_exit!(res.as_ref().err(), format!("Len {}", &stack.len()).as_str());
        res
    }
}

// impl StackItem {
//     pub fn try_allocate(self, scope: &TransactionScope, context: &mut impl TransactionContext) -> Result<Self> {
//         match self {
//             StackItem::BigMap(big_map) => {
//                 big_map
//                     .try_allocate(&scope.self_address, context)
//                     .map(|i| i.into())
//             },
//             StackItem::Option(option) => {
                
//             }
//         }
//     }
// }