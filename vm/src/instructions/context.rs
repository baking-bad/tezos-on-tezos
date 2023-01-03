use tezos_michelson::michelson::data::instructions::{
    Amount, Balance, Sender, Source, Now, Level, SelfAddress, Self_, Address, Contract, ImplicitAccount, TransferTokens
};
use tezos_core::types::encoded;

use crate::{
    Result,
    interpreter::{ScopedInterpreter, TransactionScope, TransactionContext},
    types::{MutezItem, AddressItem, TimestampItem, NatItem},
    stack::Stack,
};

impl ScopedInterpreter for Amount {
    fn execute(&self, stack: &mut Stack, scope: &TransactionScope) -> Result<()> {
        let amount = MutezItem::new(scope.amount.try_into()?)?;
        stack.push(amount.into())
    }
}

impl ScopedInterpreter for Balance {
    fn execute(&self, stack: &mut Stack, scope: &TransactionScope) -> Result<()> {
        let balance = MutezItem::new(scope.balance.try_into()?)?;
        stack.push(balance.into())
    }
}

impl ScopedInterpreter for Sender {
    fn execute(&self, stack: &mut Stack, scope: &TransactionScope) -> Result<()> {
        let sender = AddressItem::new(scope.sender.clone());
        stack.push(sender.into())
    }
}

impl ScopedInterpreter for Source {
    fn execute(&self, stack: &mut Stack, scope: &TransactionScope) -> Result<()> {
        let source = AddressItem::new(encoded::Address::Implicit(scope.source.clone()));
        stack.push(source.into())
    }
}

impl ScopedInterpreter for Now {
    fn execute(&self, stack: &mut Stack, scope: &TransactionScope) -> Result<()> {
        let now = TimestampItem::new(scope.now)?;
        stack.push(now.into())
    }
}

impl ScopedInterpreter for Level {
    fn execute(&self, stack: &mut Stack, scope: &TransactionScope) -> Result<()> {
        let level = NatItem::try_from(scope.level)?;
        stack.push(level.into())
    }
}

impl ScopedInterpreter for SelfAddress {
    fn execute(&self, stack: &mut Stack, scope: &TransactionScope) -> Result<()> {
        let self_address = AddressItem::new(encoded::Address::Originated(scope.self_address.clone()));
        stack.push(self_address.into())
    }
}
