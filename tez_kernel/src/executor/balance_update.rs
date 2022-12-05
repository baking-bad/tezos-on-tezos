use std::ops::Neg;
use num_traits::ToPrimitive;
use tezos_core::types::mutez::Mutez;
use tezos_rpc::models::balance_update;

use crate::context::TezosAddress;

pub const ALLOCATION_FEE: i64 = 1000i64;

#[derive(Clone, Debug)]
pub struct BalanceUpdates {
    balance_updates: Vec<balance_update::BalanceUpdate>
}

impl BalanceUpdates {
    pub fn new() -> Self {
        Self { balance_updates: Vec::new() }
    }

    pub fn fee(contract: &impl TezosAddress, amount: &Mutez) -> Self {
        let mut res = Self::new();
        res.spend(contract, amount);
        res
    }

    pub fn unwrap(self) -> Vec<balance_update::BalanceUpdate> {
        self.balance_updates
    }

    fn push_contract_update(&mut self, contract: String, change: String) {
        self.balance_updates.push(balance_update::BalanceUpdate::Contract(balance_update::Contract {
            kind: balance_update::Kind::Contract,
            change,
            contract,
            origin: Some(balance_update::Origin::Block)
        }));
    }

    pub fn spend(&mut self, contract: &impl TezosAddress, amount: &Mutez) {
        self.push_contract_update(
            contract.to_string().into(), 
            amount.to_i64().unwrap().neg().to_string()
        )
    }

    pub fn topup(&mut self, contract: &impl TezosAddress, amount: &Mutez) {
        self.push_contract_update(
            contract.to_string().into(), 
            amount.to_string()
        )
    }
}

impl Into<Option<Vec<balance_update::BalanceUpdate>>> for BalanceUpdates {
    fn into(self) -> Option<Vec<balance_update::BalanceUpdate>> {
        if !self.balance_updates.is_empty() {
            return Some(self.balance_updates);
        }
        None
    }
}