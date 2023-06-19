use tezos_core::types::{
    encoded::{Encoded, PublicKey, ContractAddress, ScriptExprHash},
    mutez::Mutez,
    number::Nat,
};
use tezos_michelson::micheline::Micheline;
use tezos_rpc::models::operation::Operation;
use michelson_vm::interpreter::InterpreterContext;

use layered_store::{
    LayeredStore,
    store_get,
    store_get_opt,
    store_unwrap,
};

use crate::{
    context::{
        TezosContext,
        head::Head,
        batch::BatchReceipt,
        codec::TezosStoreType,
    },
    Result
};

pub struct TezosStore<T>(pub T);

impl<T: LayeredStore<TezosStoreType>> TezosContext for TezosStore<T> {
    fn get_head(&mut self) -> Result<Head> {
        store_get!(self.0, Head::default(), "/head")
    }

    fn set_head(&mut self, head: Head) -> Result<()> {
        self.0.set("/head".into(), Some(head.into()))?;
        Ok(())
    }

    fn get_balance(&mut self, address: &str) -> Result<Option<Mutez>> {
        store_get_opt!(self.0, "/context/contracts/{}/balance", address)
    }

    fn set_balance(&mut self, address: &str, balance: Mutez) -> Result<()> {
        self.0.set(
            format!("/context/contracts/{}/balance", address),
            Some(balance.into()),
        )?;
        Ok(())
    }

    fn get_counter(&mut self, address: &str) -> Result<Nat> {
        store_get!(self.0, 0u32.into(), "/context/contracts/{}/counter", address)
    }

    fn set_counter(&mut self, address: &str, counter: Nat) -> Result<()> {
        self.0.set(
            format!("/context/contracts/{}/counter", address),
            Some(counter.into()),
        )?;
        Ok(())
    }

    fn get_public_key(&mut self, address: &str) -> Result<Option<PublicKey>> {
        store_get_opt!(self.0, "/context/contracts/{}/pubkey", address)
    }

    fn set_public_key(&mut self, address: &str, public_key: PublicKey) -> Result<()> {
        // NOTE: Underscores are not allowed in path (host restriction)
        self.0.set(
            format!("/context/contracts/{}/pubkey", address),
            Some(public_key.into()),
        )?;
        Ok(())
    }

    fn has_public_key(&self, address: &str) -> Result<bool> {
        let res = self.0.has(format!("/context/contracts/{}/pubkey", address))?;
        Ok(res)
    }

    fn set_batch_receipt(&mut self, receipt: BatchReceipt) -> Result<()> {
        self.0.set(
            format!("/batches/{}", receipt.hash.value()).into(),
            Some(receipt.into()),
        )?;
        Ok(())
    }

    fn get_batch_receipt(&mut self, hash: &str) -> Result<BatchReceipt> {
        store_unwrap!(self.0, "/batches/{}", hash)
    }

    fn set_operation_receipt(&mut self, receipt: Operation) -> Result<()> {
        self.0.set(
            format!(
                "/operations/{}",
                receipt.hash.as_ref().expect("Operation hash").value()
            ),
            Some(receipt.into()),
        )?;
        Ok(())
    }

    fn get_operation_receipt(&mut self, hash: &str) -> Result<Operation> {
        store_unwrap!(self.0, "/operations/{}", hash)
    }

    fn get_contract_code(&mut self, address: &str) -> Result<Option<Micheline>> {
        store_get_opt!(self.0, "/context/contracts/{}/code", address)
    }

    fn set_contract_code(&mut self, address: &str, code: Micheline) -> Result<()> {
        // TODO: support splitting into chunks (generic read/write loop)
        self.0.set(
            format!("/context/contracts/{}/code", address),
            Some(code.into()),
        )?;
        Ok(())
    }

    fn get_contract_storage(&mut self, address: &str) -> Result<Option<Micheline>> {
        store_get_opt!(self.0, "/context/contracts/{}/storage", address)
    }

    fn set_contract_storage(&mut self, address: &str, storage: Micheline) -> Result<()> {
        self.0.set(
            format!("/context/contracts/{}/storage", address),
            Some(storage.into()),
        )?;
        Ok(())
    }

    fn check_no_pending_changes(&self) -> Result<()> {
        if self.0.has_pending_changes() {
            Err(layered_store::Error::ContextUnstagedError.into())
        } else {
            Ok(())
        }
    }

    fn commit(&mut self) -> Result<()> {
        self.0.commit()?;
        Ok(())
    }

    fn rollback(&mut self) {
        self.0.rollback()
    }

    fn log(&mut self, msg: String) {
        self.0.log(msg)
    }
}


impl<T: LayeredStore<TezosStoreType>> InterpreterContext for TezosStore<T> {
    fn set_contract_type(&mut self, address: ContractAddress, value: Micheline) -> michelson_vm::Result<()> {
        self.0.set(
            format!("/context/contracts/{}/entrypoints", address.value()),
            Some(value.into()),
        ).map_err(michelson_vm::error::err_into)
    }

    fn get_contract_type(&mut self, address: &ContractAddress) -> michelson_vm::Result<Option<Micheline>> {
        match self.0.get(format!("/context/contracts/{}/entrypoints", address.value())) {
            Ok(Some(value)) => Ok(Some(value.try_into()?)),
            Ok(None) => Ok(None),
            Err(err) => Err(michelson_vm::error::err_into(err))
        }
    }

    fn allocate_big_map(&mut self, owner: ContractAddress) -> michelson_vm::Result<i64> {
        let ptr = match self.0.get("/context/ptr".into()) {
            Ok(Some(val)) => i64::try_from(val)? + 1,
            Ok(None) => 0i64,
            Err(err) => return Err(michelson_vm::error::err_into(err)),
        };
        self.0.set("/context/ptr".into(), Some(ptr.into()))
            .map_err(michelson_vm::error::err_into)?;
        self.0
            .set(
                format!("/context/bigmaps/{}/owner", ptr),
                Some(owner.into()),
            )
            .map_err(michelson_vm::error::err_into)?;
        Ok(ptr)
    }

    fn get_big_map_owner(&mut self, ptr: i64) -> michelson_vm::Result<Option<ContractAddress>> {
        match self.0.get(format!("/context/bigmaps/{}/owner", ptr)) {
            Ok(Some(value)) => Ok(Some(value.try_into()?)),
            Ok(None) => Ok(None),
            Err(err) => Err(michelson_vm::error::err_into(err))
        }
    }

    fn has_big_map_value(&mut self, ptr: i64, key_hash: &ScriptExprHash) -> michelson_vm::Result<bool> {
        self.0.has(format!(
            "/context/bigmaps/{}/values/{}",
            ptr,
            key_hash.value()
        )).map_err(michelson_vm::error::err_into)
    }

    fn get_big_map_value(
        &mut self,
        ptr: i64,
        key_hash: &ScriptExprHash,
    ) -> michelson_vm::Result<Option<Micheline>> {
        match self.0.get(format!("/context/bigmaps/{}/values/{}", ptr, key_hash.value())) {
            Ok(Some(value)) => Ok(Some(value.try_into()?)),
            Ok(None) => Ok(None),
            Err(err) => Err(michelson_vm::error::err_into(err))
        }
    }

    fn set_big_map_value(
        &mut self,
        ptr: i64,
        key_hash: ScriptExprHash,
        value: Option<Micheline>,
    ) -> michelson_vm::Result<()> {
        self.0.set(
            format!("/context/bigmaps/{}/values/{}", ptr, key_hash.value()),
            value.map(|v| v.into()),
        ).map_err(michelson_vm::error::err_into)
    }
}