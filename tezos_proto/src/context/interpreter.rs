use layered_store::LayeredStore;
use michelson_vm::interpreter::InterpreterContext;
use tezos_core::types::encoded::{ContractAddress, Encoded, ScriptExprHash};
use tezos_michelson::micheline::Micheline;

use crate::context::{CtxRef, TezosStoreType};

impl<T: LayeredStore<TezosStoreType>> InterpreterContext for CtxRef<T> {
    fn set_contract_type(
        &mut self,
        address: ContractAddress,
        value: Micheline,
    ) -> michelson_vm::Result<()> {
        self.set(
            format!("/context/contracts/{}/entrypoints", address.value()),
            Some(value.into()),
        )
        .map_err(michelson_vm::error::err_into)
    }

    fn get_contract_type(
        &mut self,
        address: &ContractAddress,
    ) -> michelson_vm::Result<Option<Micheline>> {
        match self.get(format!(
            "/context/contracts/{}/entrypoints",
            address.value()
        )) {
            Ok(Some(value)) => Ok(Some(value.try_into()?)),
            Ok(None) => Ok(None),
            Err(err) => Err(michelson_vm::error::err_into(err)),
        }
    }

    fn allocate_big_map(&mut self, owner: ContractAddress) -> michelson_vm::Result<i64> {
        let ptr = match self.get("/context/ptr".into()) {
            Ok(Some(val)) => i64::try_from(val)? + 1,
            Ok(None) => 0i64,
            Err(err) => return Err(michelson_vm::error::err_into(err)),
        };
        self.set("/context/ptr".into(), Some(ptr.into()))
            .map_err(michelson_vm::error::err_into)?;
        self.set(
            format!("/context/bigmaps/{}/owner", ptr),
            Some(owner.into()),
        )
        .map_err(michelson_vm::error::err_into)?;
        Ok(ptr)
    }

    fn get_big_map_owner(&mut self, ptr: i64) -> michelson_vm::Result<Option<ContractAddress>> {
        match self.get(format!("/context/bigmaps/{}/owner", ptr)) {
            Ok(Some(value)) => Ok(Some(value.try_into()?)),
            Ok(None) => Ok(None),
            Err(err) => Err(michelson_vm::error::err_into(err)),
        }
    }

    fn has_big_map_value(
        &mut self,
        ptr: i64,
        key_hash: &ScriptExprHash,
    ) -> michelson_vm::Result<bool> {
        self.has(format!(
            "/context/bigmaps/{}/values/{}",
            ptr,
            key_hash.value()
        ))
        .map_err(michelson_vm::error::err_into)
    }

    fn get_big_map_value(
        &mut self,
        ptr: i64,
        key_hash: &ScriptExprHash,
    ) -> michelson_vm::Result<Option<Micheline>> {
        match self.get(format!(
            "/context/bigmaps/{}/values/{}",
            ptr,
            key_hash.value()
        )) {
            Ok(Some(value)) => Ok(Some(value.try_into()?)),
            Ok(None) => Ok(None),
            Err(err) => Err(michelson_vm::error::err_into(err)),
        }
    }

    fn set_big_map_value(
        &mut self,
        ptr: i64,
        key_hash: ScriptExprHash,
        value: Option<Micheline>,
    ) -> michelson_vm::Result<()> {
        self.set(
            format!("/context/bigmaps/{}/values/{}", ptr, key_hash.value()),
            value.map(|v| v.into()),
        )
        .map_err(michelson_vm::error::err_into)
    }
}
