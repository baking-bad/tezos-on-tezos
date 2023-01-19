use tezos_core::types::mutez::Mutez;
use tezos_rpc::models::balance_update::{
    BalanceUpdate, Contract, Kind, Origin,
};

use crate::{
    context::proto::ProtoContext,
    Result,
    Error
};

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

    pub fn fee(contract: &str, amount: &Mutez) -> Self {
        let mut res = Self::new();
        res.spend(contract, amount);
        res
    }

    pub fn unwrap(self) -> Vec<BalanceUpdate> {
        self.balance_updates
    }

    fn push_contract_update(&mut self, contract: String, change: String) {
        self.balance_updates.push(BalanceUpdate::Contract(Contract {
            kind: Kind::Contract,
            change,
            contract,
            origin: Some(Origin::Block),
        }));
    }

    pub fn transfer(&mut self, context: &mut impl ProtoContext, source: &str, destination: &str, amount: &Mutez) -> Result<(Mutez, Mutez)> {
        let mut src_balance = context
            .get_balance(source)?
            .ok_or(Error::BalanceNotInitialized)?;
        
        let mut dst_balance = context
            .get_balance(destination)?
            .unwrap_or(0u32.into());

        if src_balance < *amount {
            return Err(Error::BalanceTooLow { balance: src_balance });
        }

        src_balance -= *amount;
        dst_balance += *amount;

        context.set_balance(source, &src_balance)?;
        context.set_balance(destination, &dst_balance)?;
        
        self.push_contract_update(source.to_string().into(), format!("-{}", amount));
        self.push_contract_update(destination.to_string().into(), amount.to_string());

        Ok((src_balance, dst_balance))
    }

    pub fn spend(&mut self, contract: &str, amount: &Mutez) {
        self.push_contract_update(contract.to_string().into(), format!("-{}", amount))
    }

    pub fn topup(&mut self, contract: &str, amount: &Mutez) {
        self.push_contract_update(contract.to_string().into(), amount.to_string())
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
