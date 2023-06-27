// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use layered_store::{LayeredStore, StoreBackend};
use tezos_core::types::encoded::{ContractAddress, Encoded, ScriptExprHash};
use tezos_michelson::micheline::Micheline;

use crate::{error::err_into, InterpreterContext, Result};

impl<Backend: StoreBackend> InterpreterContext for LayeredStore<Backend> {
    fn set_contract_type(&mut self, address: ContractAddress, value: Micheline) -> Result<()> {
        self.set(
            format!("/context/contracts/{}/entrypoints", address.value()),
            Some(value),
        )
        .map_err(err_into)
    }

    fn get_contract_type(&mut self, address: &ContractAddress) -> Result<Option<Micheline>> {
        self.get(format!(
            "/context/contracts/{}/entrypoints",
            address.value()
        ))
        .map_err(err_into)
    }

    fn allocate_big_map(&mut self, owner: ContractAddress) -> Result<i64> {
        let ptr = match self.get::<i64>("/context/ptr".into()) {
            Ok(Some(val)) => val + 1,
            Ok(None) => 0i64,
            Err(err) => return Err(err_into(err)),
        };
        self.set("/context/ptr".into(), Some(ptr))
            .map_err(err_into)?;
        self.set(format!("/context/bigmaps/{}/owner", ptr), Some(owner))
            .map_err(err_into)?;
        Ok(ptr)
    }

    fn get_big_map_owner(&mut self, ptr: i64) -> Result<Option<ContractAddress>> {
        self.get(format!("/context/bigmaps/{}/owner", ptr))
            .map_err(err_into)
    }

    fn has_big_map_value(&mut self, ptr: i64, key_hash: &ScriptExprHash) -> Result<bool> {
        self.has(format!(
            "/context/bigmaps/{}/values/{}",
            ptr,
            key_hash.value()
        ))
        .map_err(err_into)
    }

    fn get_big_map_value(
        &mut self,
        ptr: i64,
        key_hash: &ScriptExprHash,
    ) -> Result<Option<Micheline>> {
        self.get(format!(
            "/context/bigmaps/{}/values/{}",
            ptr,
            key_hash.value()
        ))
        .map_err(err_into)
    }

    fn set_big_map_value(
        &mut self,
        ptr: i64,
        key_hash: ScriptExprHash,
        value: Option<Micheline>,
    ) -> Result<()> {
        self.set(
            format!("/context/bigmaps/{}/values/{}", ptr, key_hash.value()),
            value,
        )
        .map_err(err_into)
    }
}
