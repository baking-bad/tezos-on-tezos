use std::collections::HashMap;
use tezos_michelson::micheline::{
    Micheline,
    primitive_application,
};
use tezos_core::types::{
    mutez::Mutez,
    encoded::{self, Encoded}
};
use vm::{
    Result,
    Error,
    interpreter::{TransactionScope, TransactionContext}
};

pub const CHAIN_ID: &str = "NetXP2FfcNxFANL";
pub const DEFAULT_ORIGINATED_ADDRESS: &str = "KT1BEqzn5Wx8uJrZNvuS9DVHmLvG9td3fDLi";
pub const DEFAULT_IMPLICIT_ADDRESS: &str = "tz1Ke2h7sDdakHJQh8WX4Z372du1KChsksyU";

pub fn default_scope() -> TransactionScope {
    TransactionScope {
        chain_id: CHAIN_ID.try_into().unwrap(),
        amount: 0u32.into(),
        level: 0.into(),
        now: 0,
        entrypoint: "default".into(),
        parameter: Micheline::PrimitiveApplication(primitive_application("Unit")),
        storage: Micheline::PrimitiveApplication(primitive_application("Unit")),
        self_address: DEFAULT_ORIGINATED_ADDRESS.try_into().unwrap(),
        sender: DEFAULT_IMPLICIT_ADDRESS.try_into().unwrap(),
        source: DEFAULT_IMPLICIT_ADDRESS.try_into().unwrap(),
    }
}

pub struct MockContext {
    pub balance: Mutez,
    pub big_map_counter: i64,
    pub big_maps: HashMap<i64, encoded::Address>,
    pub contracts: HashMap<String, Micheline>,
}

impl MockContext {
    pub fn default() -> Self {
        Self {
            balance: 0u8.try_into().unwrap(),
            big_map_counter: 0,
            big_maps: HashMap::new(),
            contracts: HashMap::new()
        }
    }
}

impl TransactionContext for MockContext {
    fn get_balance(&self, address: &encoded::Address) -> Result<Option<Mutez>> {
        Ok(Some(self.balance))
    }

    fn get_contract_type(&self, address: &encoded::ContractAddress) -> Result<Option<Micheline>> {
        let key = address.into_string();
        match self.contracts.get(&key) {
            Some(ty) => Ok(Some(ty.clone())),
            None => Ok(None)
        }
    }

    fn allocate_big_map(&mut self, owner: encoded::Address) -> Result<i64> {
        self.big_map_counter += 1;
        self.big_maps.insert(self.big_map_counter.clone(), owner);
        Ok(self.big_map_counter)
    }

    fn move_big_map(&mut self, ptr: i64, owner: encoded::Address) -> Result<()> {
        match self.big_maps.remove(&ptr) {
            Some(_) => {
                self.big_maps.insert(ptr, owner);
                Ok(())
            },
            None => Err(Error::BigMapNotAllocated { ptr: ptr })
        }
    }

    fn has_big_map_value(&self, ptr: i64, key_hash: &encoded::ScriptExprHash) -> Result<bool> {
        Ok(false)
    }

    fn get_big_map_value(&self, ptr: i64, key_hash: &encoded::ScriptExprHash) -> Result<Option<Micheline>> {
        Ok(None)
    }

    fn set_big_map_value(&mut self, ptr: i64, key_hash: encoded::ScriptExprHash, value: Option<Micheline>) -> Result<()> {
        Ok(())
    }
}