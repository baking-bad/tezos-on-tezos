use tezos_rpc::models::error::RpcError;
use tezos_core::types::mutez::Mutez;

use crate::context::node::TezosAddress;

pub const ALLOCATION_FEE: u32 = 1000u32;

#[derive(Clone, Debug)]
pub struct RuntimeErrors {
    errors: Vec<RpcError>
}

impl RuntimeErrors {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn unwrap(self) -> Vec<RpcError> {
        self.errors
    }

    pub fn balance_too_low(&mut self, amount: &Mutez, balance: &Mutez, contract: &impl TezosAddress) {
        self.errors.push(RpcError {
            kind: "temporary".into(),
            id: "contract.balance_too_low".into(),
            amount: Some(amount.to_string()),
            balance: Some(balance.to_string()),
            contract: Some(contract.to_string().into()),
            message: None,
            msg: None
        })
    }

    pub fn cannot_pay_storage_fee(&mut self, balance: &Mutez, contract: &impl TezosAddress) {
        self.errors.push(RpcError {
            kind: "temporary".into(),
            id: "contract.cannot_pay_storage_fee".into(),
            amount: Some(ALLOCATION_FEE.to_string()),
            balance: Some(balance.to_string()),
            contract: Some(contract.to_string().into()),
            message: None,
            msg: None
        })
    }

    pub fn previously_revealed_key(&mut self, contract: &impl TezosAddress) {
        self.errors.push(RpcError {
            kind: "temporary".into(),
            id: "contract.previously_revealed_key".into(),
            amount: None,
            balance: None,
            contract: Some(contract.to_string().into()),
            message: None,
            msg: None
        })
    }
}

impl Into<Option<Vec<RpcError>>> for RuntimeErrors {
    fn into(self) -> Option<Vec<RpcError>> {
        if !self.errors.is_empty() {
            return Some(self.errors);
        }
        None
    }
}