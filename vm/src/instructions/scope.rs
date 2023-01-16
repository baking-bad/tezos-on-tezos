use tezos_michelson::michelson::data::instructions::{
    Amount, Sender, Source, Now, Level, SelfAddress, ChainId, Self_, Balance
};
use tezos_core::types::{
    encoded
};

use crate::{
    Result,
    interpreter::{ScopedInterpreter, OperationScope},
    types::{MutezItem, AddressItem, TimestampItem, NatItem, ChainIdItem, ContractItem},
    stack::Stack,
};

impl ScopedInterpreter for Amount {
    fn execute(&self, stack: &mut Stack, scope: &OperationScope) -> Result<()> {
        let amount = MutezItem::new(scope.amount.try_into()?)?;
        stack.push(amount.into())
    }
}

impl ScopedInterpreter for Sender {
    fn execute(&self, stack: &mut Stack, scope: &OperationScope) -> Result<()> {
        let sender = AddressItem::new(scope.sender.clone());
        stack.push(sender.into())
    }
}

impl ScopedInterpreter for Source {
    fn execute(&self, stack: &mut Stack, scope: &OperationScope) -> Result<()> {
        let source = AddressItem::new(encoded::Address::Implicit(scope.source.clone()));
        stack.push(source.into())
    }
}

impl ScopedInterpreter for Now {
    fn execute(&self, stack: &mut Stack, scope: &OperationScope) -> Result<()> {
        let now = TimestampItem::new(scope.now)?;
        stack.push(now.into())
    }
}

impl ScopedInterpreter for Level {
    fn execute(&self, stack: &mut Stack, scope: &OperationScope) -> Result<()> {
        let level = NatItem::try_from(scope.level)?;
        stack.push(level.into())
    }
}

impl ScopedInterpreter for ChainId {
    fn execute(&self, stack: &mut Stack, scope: &OperationScope) -> Result<()> {
        let chain_id = ChainIdItem::new(scope.chain_id.clone());
        stack.push(chain_id.into())
    }
}

impl ScopedInterpreter for SelfAddress {
    fn execute(&self, stack: &mut Stack, scope: &OperationScope) -> Result<()> {
        let self_address = AddressItem::new(encoded::Address::Originated(scope.self_address.clone()));
        stack.push(self_address.into())
    }
}

impl ScopedInterpreter for Self_ {
    fn execute(&self, stack: &mut Stack, scope: &OperationScope) -> Result<()> {
        let self_ = ContractItem::new(
            scope.self_address.clone().into(), 
            scope.self_type.clone().try_into()?
        );
        stack.push(self_.into())
    }
}

impl ScopedInterpreter for Balance {
    fn execute(&self, stack: &mut Stack, scope: &OperationScope) -> Result<()> {
        let balance = MutezItem::new(scope.balance.try_into()?)?;
        stack.push(balance.into())
    }
}