use tezos_core::types::encoded::{ContractAddress, Encoded, ScriptExprHash};
use tezos_michelson::micheline::Micheline;

use crate::{context_get_opt, GenericContext, InterpreterContext, Result};

impl<T: GenericContext> InterpreterContext for T {
    fn set_contract_type(&mut self, address: ContractAddress, value: Micheline) -> Result<()> {
        self.set(
            format!("/context/contracts/{}/entrypoints", address.value()),
            Some(value.into()),
        )
    }

    fn get_contract_type(&mut self, address: &ContractAddress) -> Result<Option<Micheline>> {
        context_get_opt!(self, "/context/contracts/{}/entrypoints", address.value())
    }

    fn allocate_big_map(&mut self, owner: ContractAddress) -> Result<i64> {
        let ptr = match self.get("/context/ptr".into()) {
            Ok(Some(val)) => i64::try_from(val)? + 1,
            Ok(None) => 0i64,
            Err(err) => return Err(err),
        };
        self.set("/context/ptr".into(), Some(ptr.into()))?;
        self.set(
            format!("/context/bigmaps/{}/owner", ptr),
            Some(owner.into()),
        )?;
        Ok(ptr)
    }

    fn get_big_map_owner(&mut self, ptr: i64) -> Result<Option<ContractAddress>> {
        context_get_opt!(self, "/context/bigmaps/{}/owner", ptr)
    }

    fn has_big_map_value(&mut self, ptr: i64, key_hash: &ScriptExprHash) -> Result<bool> {
        self.has(format!(
            "/context/bigmaps/{}/values/{}",
            ptr,
            key_hash.value()
        ))
    }

    fn get_big_map_value(
        &mut self,
        ptr: i64,
        key_hash: &ScriptExprHash,
    ) -> Result<Option<Micheline>> {
        context_get_opt!(self, "/context/bigmaps/{}/values/{}", ptr, key_hash.value())
    }

    fn set_big_map_value(
        &mut self,
        ptr: i64,
        key_hash: ScriptExprHash,
        value: Option<Micheline>,
    ) -> Result<()> {
        self.set(
            format!("/context/bigmaps/{}/values/{}", ptr, key_hash.value()),
            value.map(|v| v.into()),
        )
    }
}
