pub use tezos_rpc::models::error::RpcError;
use tezos_core::types::mutez::Mutez;

use crate::context::types::TezosAddress;
use crate::constants::ALLOCATION_FEE;

#[derive(Clone, Debug)]
pub struct RpcErrors {
    errors: Vec<RpcError>
}

const DEFAULT_ERROR: RpcError = RpcError {
    amount: None,
    balance: None,
    contract: None,
    id: String::new(),
    kind: String::new(),
    message: None,
    msg: None
};

impl RpcErrors {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn unwrap(self) -> Vec<RpcError> {
        self.errors
    }

    pub fn unrevealed_key(contract: &impl TezosAddress) -> RpcError {
        RpcError {
            kind: "temporary".into(),
            id: "contract.unrevealed_key".into(),
            contract: Some(contract.to_string().into()),
            ..DEFAULT_ERROR
        }
    }

    pub fn inconsistent_sources() -> RpcError {
        RpcError {
            kind: "permanent".into(),
            id: "validate.operation.inconsistent_sources".into(),
            ..DEFAULT_ERROR
        }
    }

    pub fn contents_list_error() -> RpcError {
        RpcError {
            kind: "temporary".into(),
            id: "operation.contents_list_error".into(),
            ..DEFAULT_ERROR
        }
    }

    pub fn invalid_signature() -> RpcError {
        RpcError {
            kind: "temporary".into(),
            id: "operation.invalid_signature".into(),
            ..DEFAULT_ERROR
        }
    }

    pub fn empty_implicit_contract(contract: &impl TezosAddress) -> RpcError {
        RpcError {
            kind: "temporary".into(),
            id: "implicit.empty_implicit_contract".into(),
            contract: Some(contract.to_string().into()),
            ..DEFAULT_ERROR
        }
    }

    pub fn contract_balance_too_low(amount: &Mutez, balance: &Mutez, contract: &impl TezosAddress) -> RpcError {
        RpcError {
            kind: "temporary".into(),
            id: "contract.balance_too_low".into(),
            amount: Some(amount.to_string()),
            balance: Some(balance.to_string()),
            contract: Some(contract.to_string().into()),
            ..DEFAULT_ERROR
        }
    }

    pub fn counter_in_the_past(contract: &impl TezosAddress, expected: u64, found: u64) -> RpcError {
        RpcError {
            kind: "temporary".into(),
            id: "contract.counter_in_the_past".into(),
            contract: Some(contract.to_string().into()),
            message: Some(format!("Expected {}, found {}", expected, found)),
            ..DEFAULT_ERROR
        }
    }

    pub fn inconsistent_hash(&mut self, contract: &impl TezosAddress) {
        self.errors.push(RpcError {
            kind: "temporary".into(),
            id: "contract.manager.inconsistent_hash".into(),
            contract: Some(contract.to_string().into()),
            ..DEFAULT_ERROR
        })
    }

    pub fn balance_too_low(&mut self, amount: &Mutez, balance: &Mutez, contract: &impl TezosAddress) {
        self.errors.push(Self::contract_balance_too_low(amount, balance, contract));
    }

    pub fn cannot_pay_storage_fee(&mut self, balance: &Mutez, contract: &impl TezosAddress) {
        self.errors.push(RpcError {
            kind: "temporary".into(),
            id: "contract.cannot_pay_storage_fee".into(),
            amount: Some(ALLOCATION_FEE.to_string()),
            balance: Some(balance.to_string()),
            contract: Some(contract.to_string().into()),
            ..DEFAULT_ERROR
        })
    }

    pub fn previously_revealed_key(&mut self, contract: &impl TezosAddress) {
        self.errors.push(RpcError {
            kind: "temporary".into(),
            id: "contract.previously_revealed_key".into(),
            contract: Some(contract.to_string().into()),
            ..DEFAULT_ERROR
        })
    }
}

impl Into<Option<Vec<RpcError>>> for RpcErrors {
    fn into(self) -> Option<Vec<RpcError>> {
        if !self.errors.is_empty() {
            return Some(self.errors);
        }
        None
    }
}