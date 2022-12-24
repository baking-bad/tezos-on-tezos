use tezos_core::types::mutez::Mutez;
use tezos_rpc::models::balance_update::{
    Kind,
    Category,
    Contract,
    CategorizedBalanceUpdate,
    BalanceUpdate,
    Origin
};

use crate::context::types::TezosAddress;

#[derive(Clone, Debug)]
pub struct BalanceUpdates {
    balance_updates: Vec<BalanceUpdate>
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

    pub fn unwrap(self) -> Vec<BalanceUpdate> {
        self.balance_updates
    }

    fn push_contract_update(&mut self, contract: String, change: String) {
        self.balance_updates.push(BalanceUpdate::Contract(Contract {
            kind: Kind::Contract,
            change,
            contract,
            origin: Some(Origin::Block)
        }));
    }

    fn _push_categorized_update(&mut self, kind: Kind, category: Category, change: String) {
        self.balance_updates.push(BalanceUpdate::Categorized(CategorizedBalanceUpdate {
            kind,
            category,
            change,
            origin: Some(Origin::Block),
            cycle: None,
            delegate: None,
            level: None,
            participation: None,
            revelation: None
        }));
    }

    pub fn spend(&mut self, contract: &impl TezosAddress, amount: &Mutez) {
        self.push_contract_update(
            contract.to_string().into(), 
            format!("-{}", amount)
        )
    }

    pub fn topup(&mut self, contract: &impl TezosAddress, amount: &Mutez) {
        self.push_contract_update(
            contract.to_string().into(), 
            amount.to_string()
        )
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