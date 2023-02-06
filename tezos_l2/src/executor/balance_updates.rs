use tezos_core::types::mutez::Mutez;
use tezos_ctx::ExecutorContext;
use tezos_rpc::models::balance_update::{BalanceUpdate, Contract, Kind, Origin};

use crate::{Error, Result};

#[derive(Clone, Debug)]
pub struct BalanceUpdates {
    balance_updates: Vec<BalanceUpdate>,
}

impl BalanceUpdates {
    pub fn new() -> Self {
        Self {
            balance_updates: Vec::new(),
        }
    }

    pub fn unwrap(self) -> Vec<BalanceUpdate> {
        self.balance_updates
    }

    pub fn fee(source: &str, amount: &Mutez) -> Vec<BalanceUpdate> {
        let mut res = Self::new();
        res.push_contract_update(source, format!("-{}", amount));
        // TODO: res.push_contract_update(producer, amount.to_string());
        res.balance_updates
    }

    fn push_contract_update(&mut self, contract: &str, change: String) {
        self.balance_updates.push(BalanceUpdate::Contract(Contract {
            kind: Kind::Contract,
            change,
            contract: contract.to_string(),
            origin: Some(Origin::Block),
        }));
    }

    pub fn transfer(
        &mut self,
        context: &mut impl ExecutorContext,
        source: &str,
        destination: &str,
        amount: &Mutez,
    ) -> Result<(Mutez, Mutez)> {
        let mut src_balance = context
            .get_balance(source)?
            .ok_or(Error::BalanceNotInitialized)?;

        let mut dst_balance = context.get_balance(destination)?.unwrap_or(0u32.into());

        if src_balance < *amount {
            return Err(Error::BalanceTooLow {
                balance: src_balance,
            });
        }

        src_balance -= *amount;
        dst_balance += *amount;

        context.set_balance(source, src_balance.clone())?;
        context.set_balance(destination, dst_balance.clone())?;

        self.push_contract_update(source, format!("-{}", amount));
        self.push_contract_update(destination, amount.to_string());

        Ok((src_balance, dst_balance))
    }

    pub fn reserve(
        context: &mut impl ExecutorContext,
        source: &str,
        amount: &Mutez,
    ) -> Result<Mutez> {
        let mut src_balance = context
            .get_balance(source)?
            .ok_or(Error::BalanceNotInitialized)?;

        if src_balance < *amount {
            return Err(Error::BalanceTooLow {
                balance: src_balance,
            });
        }

        src_balance -= *amount;

        context.set_balance(source, src_balance.clone())?;

        Ok(src_balance)
    }
}

impl Into<Option<Vec<BalanceUpdate>>> for BalanceUpdates {
    fn into(self) -> Option<Vec<BalanceUpdate>> {
        if !self.balance_updates.is_empty() {
            return Some(self.balance_updates);
        }
        None
    }
}
