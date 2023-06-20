use tezos_core::types::{
    encoded::{PublicKey, Encoded},
    mutez::Mutez,
    number::Nat,
};
use tezos_michelson::micheline::Micheline;
use tezos_rpc::models::operation::Operation;
use layered_store::{LayeredStore, store_get, store_get_opt, store_unwrap};

use crate::{
    context::{
        head::Head,
        batch::BatchReceipt,
        TezosStoreType,
        CtxRef
    },
    Result
};

pub trait TezosContext {
    fn get_head(&mut self) -> Result<Head>;
    fn set_head(&mut self, head: Head) -> Result<()>;
    fn get_balance(&mut self, address: &str) -> Result<Option<Mutez>>;
    fn set_balance(&mut self, address: &str, balance: Mutez) -> Result<()>;
    fn get_counter(&mut self, address: &str) -> Result<Nat>;
    fn set_counter(&mut self, address: &str, counter: Nat) -> Result<()>;
    fn has_public_key(&self, address: &str) -> Result<bool>;
    fn get_public_key(&mut self, address: &str) -> Result<Option<PublicKey>>;
    fn set_public_key(&mut self, address: &str, public_key: PublicKey) -> Result<()>;
    fn set_contract_code(&mut self, address: &str, code: Micheline) -> Result<()>;
    fn get_contract_code(&mut self, address: &str) -> Result<Option<Micheline>>;
    fn get_contract_storage(&mut self, address: &str) -> Result<Option<Micheline>>;
    fn set_contract_storage(&mut self, address: &str, storage: Micheline) -> Result<()>;
    fn set_batch_receipt(&mut self, receipt: BatchReceipt) -> Result<()>;
    fn get_batch_receipt(&mut self, hash: &str) -> Result<BatchReceipt>;
    fn set_operation_receipt(&mut self, receipt: Operation) -> Result<()>;
    fn get_operation_receipt(&mut self, hash: &str) -> Result<Operation>;
    fn check_no_pending_changes(&self) -> Result<()>;
    fn commit(&mut self) -> Result<()>;
    fn rollback(&mut self);
    fn log(&mut self, msg: String);
}

impl<T: LayeredStore<TezosStoreType>> TezosContext for CtxRef<T> {
    fn get_head(&mut self) -> Result<Head> {
        store_get!(self, Head::default(), "/head")
    }

    fn set_head(&mut self, head: Head) -> Result<()> {
        self.set("/head".into(), Some(head.into()))?;
        Ok(())
    }

    fn get_balance(&mut self, address: &str) -> Result<Option<Mutez>> {
        store_get_opt!(self, "/context/contracts/{}/balance", address)
    }

    fn set_balance(&mut self, address: &str, balance: Mutez) -> Result<()> {
        self.set(
            format!("/context/contracts/{}/balance", address),
            Some(balance.into()),
        )?;
        Ok(())
    }

    fn get_counter(&mut self, address: &str) -> Result<Nat> {
        store_get!(self, 0u32.into(), "/context/contracts/{}/counter", address)
    }

    fn set_counter(&mut self, address: &str, counter: Nat) -> Result<()> {
        self.set(
            format!("/context/contracts/{}/counter", address),
            Some(counter.into()),
        )?;
        Ok(())
    }

    fn get_public_key(&mut self, address: &str) -> Result<Option<PublicKey>> {
        store_get_opt!(self, "/context/contracts/{}/pubkey", address)
    }

    fn set_public_key(&mut self, address: &str, public_key: PublicKey) -> Result<()> {
        // NOTE: Underscores are not allowed in path (host restriction)
        self.set(
            format!("/context/contracts/{}/pubkey", address),
            Some(public_key.into()),
        )?;
        Ok(())
    }

    fn has_public_key(&self, address: &str) -> Result<bool> {
        let res = self.has(format!("/context/contracts/{}/pubkey", address))?;
        Ok(res)
    }

    fn set_batch_receipt(&mut self, receipt: BatchReceipt) -> Result<()> {
        self.set(
            format!("/batches/{}", receipt.hash.value()).into(),
            Some(receipt.into()),
        )?;
        Ok(())
    }

    fn get_batch_receipt(&mut self, hash: &str) -> Result<BatchReceipt> {
        store_unwrap!(self, "/batches/{}", hash)
    }

    fn set_operation_receipt(&mut self, receipt: Operation) -> Result<()> {
        self.set(
            format!(
                "/operations/{}",
                receipt.hash.as_ref().expect("Operation hash").value()
            ),
            Some(receipt.into()),
        )?;
        Ok(())
    }

    fn get_operation_receipt(&mut self, hash: &str) -> Result<Operation> {
        store_unwrap!(self, "/operations/{}", hash)
    }

    fn get_contract_code(&mut self, address: &str) -> Result<Option<Micheline>> {
        store_get_opt!(self, "/context/contracts/{}/code", address)
    }

    fn set_contract_code(&mut self, address: &str, code: Micheline) -> Result<()> {
        // TODO: support splitting into chunks (generic read/write loop)
        self.set(
            format!("/context/contracts/{}/code", address),
            Some(code.into()),
        )?;
        Ok(())
    }

    fn get_contract_storage(&mut self, address: &str) -> Result<Option<Micheline>> {
        store_get_opt!(self, "/context/contracts/{}/storage", address)
    }

    fn set_contract_storage(&mut self, address: &str, storage: Micheline) -> Result<()> {
        self.set(
            format!("/context/contracts/{}/storage", address),
            Some(storage.into()),
        )?;
        Ok(())
    }

    fn check_no_pending_changes(&self) -> Result<()> {
        if self.has_pending_changes() {
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
