// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use ibig::IBig;
use tezos_core::types::{
    encoded::{Address, ChainId, ContractAddress, ImplicitAddress, ScriptExprHash},
    mutez::Mutez,
};
use tezos_michelson::micheline::Micheline;
use tezos_michelson::michelson::{data::Instruction, types::Type};

use crate::{
    err_unsupported,
    formatter::Formatter,
    stack::Stack,
    trace_enter, trace_exit,
    types::{BigMapDiff, StackItem, TicketItem},
    Result,
};

pub trait InterpreterContext {
    fn get_contract_type(&mut self, address: &ContractAddress) -> Result<Option<Micheline>>;
    fn set_contract_type(&mut self, address: ContractAddress, value: Micheline) -> Result<()>;
    fn allocate_big_map(&mut self, owner: ContractAddress) -> Result<i64>;
    // TODO: transfer_big_map
    fn get_big_map_owner(&mut self, ptr: i64) -> Result<Option<ContractAddress>>;
    fn has_big_map_value(&mut self, ptr: i64, key_hash: &ScriptExprHash) -> Result<bool>;
    fn get_big_map_value(
        &mut self,
        ptr: i64,
        key_hash: &ScriptExprHash,
    ) -> Result<Option<Micheline>>;
    fn set_big_map_value(
        &mut self,
        ptr: i64,
        key_hash: ScriptExprHash,
        value: Option<Micheline>,
    ) -> Result<()>;
    fn update_ticket_balance(
        &mut self,
        tickiter: Address,
        identifier: Micheline,
        owner: Address,
        value: IBig,
    ) -> Result<()>;
}

pub struct OperationScope {
    pub chain_id: ChainId,
    pub source: ImplicitAddress,
    pub sender: Address,
    pub amount: Mutez,
    pub balance: Mutez,
    pub parameters: Option<(String, Micheline)>,
    pub storage: Micheline,
    pub now: i64,
    pub self_address: ContractAddress,
    pub self_type: Micheline,
    pub level: i32,
}

pub trait Interpreter {
    fn execute(
        &self,
        stack: &mut Stack,
        scope: &OperationScope,
        context: &mut impl InterpreterContext,
    ) -> Result<()>;
}

pub trait PureInterpreter {
    fn execute(&self, stack: &mut Stack) -> Result<()>;
}

pub trait ScopedInterpreter {
    fn execute(&self, stack: &mut Stack, scope: &OperationScope) -> Result<()>;
}

pub trait ContextInterpreter {
    fn execute(&self, stack: &mut Stack, context: &mut impl InterpreterContext) -> Result<()>;
}

pub trait LazyStorage {
    fn try_acquire(
        &mut self,
        owner: &ContractAddress,
        context: &mut impl InterpreterContext,
    ) -> Result<()>;
    fn try_aggregate(&mut self, output: &mut Vec<BigMapDiff>, ty: &Type) -> Result<()>;
}

pub trait TicketStorage {
    fn has_tickets(&self) -> bool;
    fn iter_tickets(&self, action: &mut impl FnMut(&TicketItem) -> Result<()>) -> Result<()>;
    fn drop_tickets(
        &self,
        owner: &ContractAddress,
        context: &mut impl InterpreterContext,
    ) -> Result<()> {
        self.iter_tickets(&mut |t| {
            let amount: IBig = t.amount.value().into();
            context.update_ticket_balance(
                t.source.clone().unwrap(),
                t.identifier
                    .clone()
                    .into_micheline(&t.identifier.get_type()?)?,
                owner.clone().into(),
                -amount,
            )
        })
    }
}

impl Interpreter for Instruction {
    fn execute(
        &self,
        stack: &mut Stack,
        scope: &OperationScope,
        context: &mut impl InterpreterContext,
    ) -> Result<()> {
        trace_enter!(self);
        let res = match self {
            Instruction::Sequence(seq) => return seq.execute(stack, scope, context),
            Instruction::Push(instr) => instr.execute(stack),
            Instruction::Drop(instr) => instr.execute(stack, scope, context),
            Instruction::Dup(instr) => instr.execute(stack),
            Instruction::Swap(instr) => instr.execute(stack),
            Instruction::Dig(instr) => instr.execute(stack),
            Instruction::Dug(instr) => instr.execute(stack),
            Instruction::Rename(_) => Ok(()),
            Instruction::Cast(instr) => instr.execute(stack),
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
            Instruction::SubMutez(instr) => instr.execute(stack),
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
            Instruction::Balance(instr) => instr.execute(stack, scope),
            Instruction::Address(instr) => instr.execute(stack),
            Instruction::Contract(instr) => instr.execute(stack, context),
            Instruction::Self_(instr) => instr.execute(stack, scope),
            Instruction::ImplicitAccount(instr) => instr.execute(stack),
            Instruction::EmptyBigMap(instr) => instr.execute(stack, scope, context),
            Instruction::TransferTokens(instr) => instr.execute(stack, scope, context),
            Instruction::Blake2B(instr) => instr.execute(stack),
            Instruction::HashKey(instr) => instr.execute(stack),
            Instruction::CheckSignature(instr) => instr.execute(stack),
            Instruction::Ticket(instr) => instr.execute(stack, scope, context),
            Instruction::ReadTicket(instr) => instr.execute(stack),
            Instruction::SplitTicket(instr) => instr.execute(stack),
            Instruction::JoinTickets(instr) => instr.execute(stack),
            _ => err_unsupported!(self.format()),
        };
        trace_exit!(res.as_ref().err(), format!("Len {}", &stack.len()).as_str());
        res
    }
}

impl LazyStorage for StackItem {
    fn try_acquire(
        &mut self,
        owner: &ContractAddress,
        context: &mut impl InterpreterContext,
    ) -> Result<()> {
        match self {
            StackItem::BigMap(item) => item.try_acquire(owner, context),
            StackItem::Option(item) => item.try_acquire(owner, context),
            StackItem::Or(item) => item.try_acquire(owner, context),
            StackItem::Pair(item) => item.try_acquire(owner, context),
            StackItem::List(item) => item.try_acquire(owner, context),
            StackItem::Map(item) => item.try_acquire(owner, context),
            _ => Ok(()),
        }
    }

    fn try_aggregate(&mut self, output: &mut Vec<BigMapDiff>, ty: &Type) -> Result<()> {
        match self {
            StackItem::BigMap(item) => item.try_aggregate(output, ty),
            StackItem::Option(item) => item.try_aggregate(output, ty),
            StackItem::Or(item) => item.try_aggregate(output, ty),
            StackItem::Pair(item) => item.try_aggregate(output, ty),
            StackItem::List(item) => item.try_aggregate(output, ty),
            StackItem::Map(item) => item.try_aggregate(output, ty),
            _ => Ok(()),
        }
    }
}
