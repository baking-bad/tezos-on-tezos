use serde_json_wasm;
use tezos_core::types::{
    encoded::{PublicKey, OperationHash, BlockHash},
    mutez::Mutez,
    number::Nat
};
use tezos_michelson::micheline::Micheline;

use crate::{
    GenericContext,
    ExecutorContext,
    Head,
    Error,
    Result,
    context_get,
    context_get_opt
};

impl<T: GenericContext> ExecutorContext for T {
    fn get_head(&mut self) -> Result<Head> {
        context_get!(self, Head::default(), "/head")
    }

    fn commit_head(&mut self, head: Head) -> Result<()> {
        self.save("/head".into(), Some(head.into()))?;
        Ok(())
    }

    fn get_balance(&mut self, address: &str) -> Result<Option<Mutez>> {
        context_get_opt!(self, "/context/contracts/{}/balance", address)
    }

    fn set_balance(&mut self, address: &str, balance: &Mutez) -> Result<()> {
        return self.set(
            format!("/context/contracts/{}/balance", address),
            Some(balance.to_owned().into()),
        );
    }

    fn get_counter(&mut self, address: &str) -> Result<Option<Nat>> {
        // TODO: use u64 or UBig instead of Nat, because it is String under the hood, not good for math
        context_get_opt!(self, "/context/contracts/{}/counter", address)
    }

    fn set_counter(&mut self, address: &str, counter: &Nat) -> Result<()> {
        return self.set(
            format!("/context/contracts/{}/counter", address),
            Some(counter.to_owned().into()),
        );
    }

    fn get_public_key(&mut self, address: &str) -> Result<Option<PublicKey>> {
        context_get_opt!(self, "/context/contracts/{}/pubkey", address)
    }

    fn set_public_key(&mut self, address: &str, public_key: &PublicKey) -> Result<()> {
        // NOTE: Underscores are not allowed in path (host restriction)
        return self.set(
            format!("/context/contracts/{}/pubkey", address),
            Some(public_key.to_owned().into()),
        );
    }

    fn has_public_key(&self, address: &str) -> Result<bool> {
        return self.has(format!("/context/contracts/{}/pubkey", address));
    }

    fn commit_operation<R: serde::Serialize>(
        &mut self,
        level: i32,
        index: i32,
        hash: OperationHash,
        receipt: R,
    ) -> Result<()> {
        self.save(
            format!("/blocks/{}/ophashes/{}", level, index),
            Some(hash.into()),
        )?;
        let receipt = serde_json_wasm::to_vec(&receipt)?; 
        self.save(
            format!("/blocks/{}/operations/{}", level, index),
            Some(receipt.into()),
        )?;
        Ok(())
    }

    fn get_operation_receipt<R: serde::de::DeserializeOwned>(&mut self, level: i32, index: i32)
            -> Result<Option<R>> {
        match self.get(format!("/blocks/{}/operations/{}", level, index)) {
            Ok(Some(bytes)) => {
                let data: Vec<u8> = bytes.try_into()?;
                let res: R = serde_json_wasm::from_slice(data.as_slice())?;
                Ok(Some(res))
            },
            Ok(None) => Ok(None),
            Err(err) => Err(err)
        }
    }

    fn commit_batch<R: serde::Serialize>(&mut self, level: i32, hash: BlockHash, receipt: R) -> Result<()> {
        self.save(
            format!("/blocks/{}/hash", level),
            Some(hash.into()),
        )?;
        let receipt = serde_json_wasm::to_vec(&receipt)?; 
        self.save(
            format!("/blocks/{}/header", level),
            Some(receipt.into()))?;
        Ok(())
    }

    fn get_batch_receipt<R: serde::de::DeserializeOwned>(&mut self, level: i32) -> Result<Option<R>> {
        match self.get(format!("/blocks/{}/header", level)) {
            Ok(Some(bytes)) => {
                let data: Vec<u8> = bytes.try_into()?;
                let res: R = serde_json_wasm::from_slice(data.as_slice())?;
                Ok(Some(res))
            },
            Ok(None) => Ok(None),
            Err(err) => Err(err)
        }
    }

    fn get_contract_code(&mut self, address: &str) -> Result<Option<Micheline>> {
        context_get_opt!(self, "/context/contracts/{}/code", address)
    }

    fn set_contract_code(&mut self, address: &str, code: Micheline) -> Result<()> {
        // TODO: support splitting into chunks (generic read/write loop)
        self.set(format!("/context/contracts/{}/code", address), Some(code.into()))
    }

    fn get_contract_storage(&mut self, address: &str) -> Result<Option<Micheline>> {
        context_get_opt!(self, "/context/contracts/{}/storage", address)
    }

    fn set_contract_storage(&mut self, address: &str, storage: Micheline) -> Result<()> {
        self.set(
            format!("/context/contracts/{}/storage", address),
            Some(storage.into()),
        )
    }

    fn commit(&mut self) -> Result<()> {
        for (key, val) in self.agg_pending_changes() {
            self.save(key, val)?;
        }
        Ok(())
    }

    fn rollback(&mut self) {
        self.agg_pending_changes();
    }

    fn check_no_pending_changes(&self) -> Result<()> {
        if self.has_pending_changes() {
            Err(Error::ContextUnstagedError)
        } else {
            Ok(())
        }
    }

    fn debug_log(&self, message: String) {
        self.log(message)
    }
}
