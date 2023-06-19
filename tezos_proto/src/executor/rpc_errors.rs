use tezos_core::types::{mutez::Mutez, number::Nat};
pub use tezos_rpc::models::error::RpcError;

#[derive(Clone, Debug)]
pub struct RpcErrors {
    errors: Vec<RpcError>,
}

const DEFAULT_ERROR: RpcError = RpcError {
    amount: None,
    balance: None,
    contract: None,
    id: String::new(),
    kind: String::new(),
    message: None,
    msg: None,
};

impl RpcErrors {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn unwrap(self) -> Vec<RpcError> {
        self.errors
    }

    pub fn unrevealed_key(&mut self, contract: &str) {
        self.errors.push(RpcError {
            kind: "temporary".into(),
            id: "contract.unrevealed_key".into(),
            contract: Some(contract.to_string().into()),
            ..DEFAULT_ERROR
        })
    }

    pub fn inconsistent_sources(&mut self) {
        self.errors.push(RpcError {
            kind: "permanent".into(),
            id: "validate.operation.inconsistent_sources".into(),
            ..DEFAULT_ERROR
        })
    }

    pub fn contents_list_error(&mut self) {
        self.errors.push(RpcError {
            kind: "temporary".into(),
            id: "operation.contents_list_error".into(),
            ..DEFAULT_ERROR
        })
    }

    pub fn invalid_signature(&mut self) {
        self.errors.push(RpcError {
            kind: "temporary".into(),
            id: "operation.invalid_signature".into(),
            ..DEFAULT_ERROR
        })
    }

    pub fn empty_implicit_contract(&mut self, contract: &str) {
        self.errors.push(RpcError {
            kind: "temporary".into(),
            id: "implicit.empty_implicit_contract".into(),
            contract: Some(contract.to_string().into()),
            ..DEFAULT_ERROR
        })
    }

    pub fn contract_balance_too_low(&mut self, amount: &Mutez, balance: &Mutez, contract: &str) {
        self.errors.push(RpcError {
            kind: "temporary".into(),
            id: "contract.balance_too_low".into(),
            amount: Some(amount.to_string()),
            balance: Some(balance.to_string()),
            contract: Some(contract.to_string().into()),
            ..DEFAULT_ERROR
        })
    }

    pub fn counter_in_the_past(&mut self, contract: &str, expected: &Nat, found: &Nat) {
        self.errors.push(RpcError {
            kind: "temporary".into(),
            id: "contract.counter_in_the_past".into(),
            contract: Some(contract.to_string().into()),
            message: Some(format!("Expected {}, found {}", expected, found)),
            ..DEFAULT_ERROR
        })
    }

    // pub fn bad_stack(message: String) -> RpcError {
    //     RpcError {
    //         kind: "temporary".into(),
    //         id: "michelson_v1.bad_stack".into(),
    //         message: Some(message),
    //         ..DEFAULT_ERROR
    //     }
    // }

    // pub fn ill_typed_data(message: String) -> RpcError {
    //     RpcError {
    //         kind: "temporary".into(),
    //         id: "michelson_v1.ill_typed_data".into(),
    //         message: Some(message),
    //         ..DEFAULT_ERROR
    //     }
    // }

    // pub fn invalid_never_expr() -> RpcError {
    //     RpcError {
    //         kind: "temporary".into(),
    //         id: "michelson_v1.invalid_never_expr".into(),
    //         ..DEFAULT_ERROR
    //     }
    // }

    // pub fn invalid_primitive(prim: String) -> RpcError {
    //     RpcError {
    //         kind: "temporary".into(),
    //         id: "michelson_v1.invalid_primitive".into(),
    //         message: Some(prim),
    //         ..DEFAULT_ERROR
    //     }
    // }

    pub fn inconsistent_hash(&mut self, contract: &str) {
        self.errors.push(RpcError {
            kind: "temporary".into(),
            id: "contract.manager.inconsistent_hash".into(),
            contract: Some(contract.to_string().into()),
            ..DEFAULT_ERROR
        })
    }

    pub fn balance_too_low(&mut self, amount: &Mutez, balance: &Mutez, contract: &str) {
        self.errors.push(RpcError {
            kind: "temporary".into(),
            id: "contract.balance_too_low".into(),
            amount: Some(amount.to_string()),
            balance: Some(balance.to_string()),
            contract: Some(contract.to_string().into()),
            ..DEFAULT_ERROR
        })
    }

    pub fn previously_revealed_key(&mut self, contract: &str) {
        self.errors.push(RpcError {
            kind: "temporary".into(),
            id: "contract.previously_revealed_key".into(),
            contract: Some(contract.to_string().into()),
            ..DEFAULT_ERROR
        })
    }

    pub fn runtime_error(&mut self, contract: &str, message: String) {
        self.errors.push(RpcError {
            kind: "temporary".into(),
            id: "michelson_v1.runtime_error".into(),
            contract: Some(contract.into()),
            message: Some(message),
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
