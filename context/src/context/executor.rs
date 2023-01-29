use tezos_core::types::{
    encoded::{PublicKey, Encoded},
    mutez::Mutez,
    number::Nat,
};
use tezos_rpc::models::operation::Operation;
use tezos_michelson::micheline::Micheline;

use crate::{
    context_get, context_get_opt, context_unwrap,
    Error, ExecutorContext, GenericContext, Head, Result, BatchReceipt
};

impl<T: GenericContext> ExecutorContext for T {
    fn get_head(&mut self) -> Result<Head> {
        context_get!(self, Head::default(), "/head")
    }

    fn set_head(&mut self, head: Head) -> Result<()> {
        self.set("/head".into(), Some(head.into()))?;
        Ok(())
    }

    fn get_balance(&mut self, address: &str) -> Result<Option<Mutez>> {
        context_get_opt!(self, "/context/contracts/{}/balance", address)
    }

    fn set_balance(&mut self, address: &str, balance: Mutez) -> Result<()> {
        return self.set(
            format!("/context/contracts/{}/balance", address),
            Some(balance.into()),
        );
    }

    fn get_counter(&mut self, address: &str) -> Result<Option<Nat>> {
        // TODO: use u64 or UBig instead of Nat, because it is String under the hood, not good for math
        context_get_opt!(self, "/context/contracts/{}/counter", address)
    }

    fn set_counter(&mut self, address: &str, counter: Nat) -> Result<()> {
        return self.set(
            format!("/context/contracts/{}/counter", address),
            Some(counter.into()),
        );
    }

    fn get_public_key(&mut self, address: &str) -> Result<Option<PublicKey>> {
        context_get_opt!(self, "/context/contracts/{}/pubkey", address)
    }

    fn set_public_key(&mut self, address: &str, public_key: PublicKey) -> Result<()> {
        // NOTE: Underscores are not allowed in path (host restriction)
        return self.set(
            format!("/context/contracts/{}/pubkey", address),
            Some(public_key.into()),
        );
    }

    fn has_public_key(&self, address: &str) -> Result<bool> {
        return self.has(format!("/context/contracts/{}/pubkey", address));
    }

    fn set_batch_receipt(&mut self, receipt: BatchReceipt) -> Result<()> {
        self.set(format!(
            "/batches/{}", receipt.hash.value()).into(),
            Some(receipt.into())
        )
    }

    fn get_batch_receipt(&mut self, hash: &str) -> Result<BatchReceipt> {
        context_unwrap!(self, "/batches/{}", hash)
    }

    fn set_operation_receipt(&mut self, receipt: Operation) -> Result<()> {
        self.set(
            format!("/operations/{}", receipt.hash.as_ref().expect("Operation hash").value()),
            Some(receipt.into()),
        )
    }

    fn get_operation_receipt(&mut self, hash: &str) -> Result<Operation> {
        context_unwrap!(self, "/operations/{}", hash)
    }

    fn get_contract_code(&mut self, address: &str) -> Result<Option<Micheline>> {
        context_get_opt!(self, "/context/contracts/{}/code", address)
    }

    fn set_contract_code(&mut self, address: &str, code: Micheline) -> Result<()> {
        // TODO: support splitting into chunks (generic read/write loop)
        self.set(
            format!("/context/contracts/{}/code", address),
            Some(code.into()),
        )
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

    fn check_no_pending_changes(&self) -> Result<()> {
        if self.has_pending_changes() {
            Err(Error::ContextUnstagedError)
        } else {
            Ok(())
        }
    }
}
