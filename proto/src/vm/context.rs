use tezos_core::types::{
    encoded::{ImplicitAddress, Address, ContractAddress},
    mutez::Mutez
};

use crate::{
    constants::*
};

pub struct ExecutionContext {
    pub source: ImplicitAddress,
    pub sender: Address,
    pub amount: Mutez,
    pub now: i64,
    pub self_address: ContractAddress,
    pub level: i32,
}

impl ExecutionContext {
    pub fn default() -> Self {
        Self {
            amount: 0u32.into(),
            level: 0,
            now: 0,
            self_address: DEFAULT_ORIGINATED_ADDRESS.try_into().unwrap(),
            sender: DEFAULT_IMPLICIT_ADDRESS.try_into().unwrap(),
            source: DEFAULT_IMPLICIT_ADDRESS.try_into().unwrap()
        }
    }
}