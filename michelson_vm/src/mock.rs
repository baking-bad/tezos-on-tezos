// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use ibig::IBig;
use std::collections::HashMap;
use tezos_core::types::{
    encoded::{self, Address, Encoded},
    mutez::Mutez,
};
use tezos_michelson::michelson::types::unit;
use tezos_michelson::{
    micheline::{primitive_application, Micheline},
    michelson::types::Type,
};

use crate::{
    interpreter::{InterpreterContext, OperationScope},
    trace_log,
    types::ticket::TicketBalanceDiff,
    Result,
};

pub const CHAIN_ID: &str = "NetXP2FfcNxFANL";
pub const DEFAULT_ORIGINATED_ADDRESS: &str = "KT1BEqzn5Wx8uJrZNvuS9DVHmLvG9td3fDLi";
pub const DEFAULT_IMPLICIT_ADDRESS: &str = "tz1Ke2h7sDdakHJQh8WX4Z372du1KChsksyU";

pub fn default_scope() -> OperationScope {
    OperationScope {
        chain_id: CHAIN_ID.try_into().unwrap(),
        amount: 0u32.into(),
        balance: 0u32.into(),
        level: 0.into(),
        now: 0,
        parameters: None,
        storage: Micheline::PrimitiveApplication(primitive_application("Unit")),
        self_address: DEFAULT_ORIGINATED_ADDRESS.try_into().unwrap(),
        self_type: unit(),
        sender: DEFAULT_IMPLICIT_ADDRESS.try_into().unwrap(),
        source: DEFAULT_IMPLICIT_ADDRESS.try_into().unwrap(),
    }
}

// TODO: use layered_store::EphemeralStore instead
pub struct MockContext {
    pub big_map_counter: i64,
    pub big_maps: HashMap<i64, encoded::ContractAddress>,
    pub big_map_values: HashMap<(i64, String), Micheline>,
    pub contracts: HashMap<String, Micheline>,
    pub balances: HashMap<String, Mutez>,
}

impl MockContext {
    pub fn default() -> Self {
        Self {
            big_map_counter: 0,
            big_maps: HashMap::new(),
            big_map_values: HashMap::new(),
            contracts: HashMap::new(),
            balances: HashMap::new(),
        }
    }
}

impl MockContext {
    pub fn get_elements_count(&self, ptr: i64) -> usize {
        self.big_map_values
            .iter()
            .filter(|((id, _), _)| id == &ptr)
            .count()
    }

    pub fn init_big_map(&mut self, ptr: i64, owner: encoded::ContractAddress) {
        trace_log!("Init", ptr);
        self.big_map_counter = ptr;
        self.big_maps.insert(ptr, owner);
    }
}

impl InterpreterContext for MockContext {
    fn set_contract_type(
        &mut self,
        address: encoded::ContractAddress,
        value: Micheline,
    ) -> Result<()> {
        let key = address.into_string();
        self.contracts.insert(key, value);
        Ok(())
    }

    fn get_contract_type(
        &mut self,
        address: &encoded::ContractAddress,
    ) -> Result<Option<Micheline>> {
        let key = address.into_string();
        match self.contracts.get(&key) {
            Some(ty) => Ok(Some(ty.clone())),
            None => Ok(None),
        }
    }

    fn allocate_big_map(&mut self, owner: encoded::ContractAddress) -> Result<i64> {
        let counter = self.big_map_counter;
        self.big_map_counter += 1;
        trace_log!("Alloc", counter);
        self.big_maps.insert(counter, owner);
        Ok(counter)
    }

    fn get_big_map_owner(&mut self, ptr: i64) -> Result<Option<encoded::ContractAddress>> {
        match self.big_maps.get(&ptr) {
            Some(owner) => Ok(Some(owner.clone())),
            None => Ok(None),
        }
    }

    fn has_big_map_value(&mut self, ptr: i64, key_hash: &encoded::ScriptExprHash) -> Result<bool> {
        trace_log!("Has", key_hash.value());
        Ok(self
            .big_map_values
            .contains_key(&(ptr, key_hash.into_string())))
    }

    fn get_big_map_value(
        &mut self,
        ptr: i64,
        key_hash: &encoded::ScriptExprHash,
    ) -> Result<Option<Micheline>> {
        trace_log!("Get", key_hash.value());
        Ok(self
            .big_map_values
            .get(&(ptr, key_hash.into_string()))
            .map(|v| v.clone()))
    }

    fn set_big_map_value(
        &mut self,
        ptr: i64,
        key_hash: encoded::ScriptExprHash,
        value: Option<Micheline>,
    ) -> Result<()> {
        trace_log!("Update", key_hash.value());
        let k = (ptr, key_hash.into_string());
        match value {
            Some(v) => self.big_map_values.insert(k, v),
            None => self.big_map_values.remove(&k),
        };
        Ok(())
    }

    fn get_ticket_balance(
        &mut self,
        tickiter: &Address,
        identifier: &Micheline,
        identifier_ty: &Type,
        owner: &Address,
    ) -> Result<ibig::UBig> {
        todo!()
    }

    fn update_ticket_balance(
        &mut self,
        tickiter: &Address,
        identifier: &Micheline,
        identifier_ty: &Type,
        owner: &Address,
        value: IBig,
    ) -> Result<()> {
        todo!()
    }

    fn aggregate_ticket_updates(&self) -> Vec<TicketBalanceDiff> {
        todo!()
    }
}
