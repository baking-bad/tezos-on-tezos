use tezos_michelson::michelson::data::instructions::{
    Amount, Sender, Source, Now, Level, SelfAddress, ChainId, TransferTokens
};
use tezos_core::types::{
    encoded
};

use crate::{
    Result,
    interpreter::{ScopedInterpreter, TransactionScope},
    types::{MutezItem, AddressItem, TimestampItem, NatItem, ChainIdItem},
    stack::Stack,
};

impl ScopedInterpreter for Amount {
    fn execute(&self, stack: &mut Stack, scope: &TransactionScope) -> Result<()> {
        let amount = MutezItem::new(scope.amount.try_into()?)?;
        stack.push(amount.into())
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

impl ScopedInterpreter for ChainId {
    fn execute(&self, stack: &mut Stack, scope: &TransactionScope) -> Result<()> {
        let chain_id = ChainIdItem::new(scope.chain_id.clone());
        stack.push(chain_id.into())
    }
}

impl ScopedInterpreter for SelfAddress {
    fn execute(&self, stack: &mut Stack, scope: &TransactionScope) -> Result<()> {
        let self_address = AddressItem::new(encoded::Address::Originated(scope.self_address.clone()));
        stack.push(self_address.into())
    }
}

impl ScopedInterpreter for TransferTokens {
    fn execute(&self, stack: &mut Stack, scope: &TransactionScope) -> Result<()> {
        Ok(())
    }
}