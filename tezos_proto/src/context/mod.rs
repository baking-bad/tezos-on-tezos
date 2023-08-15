// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

pub mod head;

use layered_store::{LayeredStore, StoreBackend};
use tezos_core::types::{
    encoded::{Encoded, PublicKey},
    mutez::Mutez,
    number::Nat,
};
use tezos_michelson::micheline::Micheline;
use tezos_operation::operations::SignedOperation;
use tezos_rpc::models::operation::Operation;

use crate::{batch::receipt::BatchReceipt, context::head::Head, error::err_into, Error, Result};

pub type TezosEphemeralContext = layered_store::EphemeralStore;

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
    fn set_pending_operation(&mut self, level: i32, operation: SignedOperation) -> Result<()>;
    fn del_pending_operation(&mut self, hash: &str) -> Result<()>;
    fn agg_pending_operations(&mut self, level: i32) -> Result<Vec<SignedOperation>>;
    fn check_no_pending_changes(&self) -> Result<()>;
    fn commit(&mut self) -> Result<()>;
    fn rollback(&mut self);
    fn log(&self, msg: String);
}

impl<Backend: StoreBackend> TezosContext for LayeredStore<Backend> {
    fn get_head(&mut self) -> Result<Head> {
        match self.get("/head".into()) {
            Ok(Some(head)) => Ok(head),
            Ok(None) => Ok(Head::default()),
            Err(err) => Err(err_into(err)),
        }
    }

    fn set_head(&mut self, head: Head) -> Result<()> {
        self.set("/head".into(), Some(head)).map_err(err_into)
    }

    fn get_balance(&mut self, address: &str) -> Result<Option<Mutez>> {
        self.get(format!("/context/contracts/{}/balance", address))
            .map_err(err_into)
    }

    fn set_balance(&mut self, address: &str, balance: Mutez) -> Result<()> {
        self.set(
            format!("/context/contracts/{}/balance", address),
            Some(balance),
        )
        .map_err(err_into)
    }

    fn get_counter(&mut self, address: &str) -> Result<Nat> {
        match self.get(format!("/context/contracts/{}/counter", address)) {
            Ok(Some(value)) => Ok(value),
            Ok(None) => Ok(Nat::from_integer(0)),
            Err(err) => Err(err_into(err)),
        }
    }

    fn set_counter(&mut self, address: &str, counter: Nat) -> Result<()> {
        self.set(
            format!("/context/contracts/{}/counter", address),
            Some(counter),
        )
        .map_err(err_into)
    }

    fn get_public_key(&mut self, address: &str) -> Result<Option<PublicKey>> {
        self.get(format!("/context/contracts/{}/pubkey", address))
            .map_err(err_into)
    }

    fn set_public_key(&mut self, address: &str, public_key: PublicKey) -> Result<()> {
        // NOTE: Underscores are not allowed in path (host restriction)
        self.set(
            format!("/context/contracts/{}/pubkey", address),
            Some(public_key),
        )
        .map_err(err_into)
    }

    fn has_public_key(&self, address: &str) -> Result<bool> {
        self.has(format!("/context/contracts/{}/pubkey", address))
            .map_err(err_into)
    }

    fn set_batch_receipt(&mut self, receipt: BatchReceipt) -> Result<()> {
        self.set(
            format!("/batches/{}", receipt.hash.value()).into(),
            Some(receipt),
        )
        .map_err(err_into)
    }

    fn get_batch_receipt(&mut self, hash: &str) -> Result<BatchReceipt> {
        self.get(format!("/batches/{}", hash))
            .map_err(err_into)?
            .ok_or(Error::BatchNotFound { hash: hash.into() })
    }

    fn set_operation_receipt(&mut self, receipt: Operation) -> Result<()> {
        self.set(
            format!(
                "/operations/{}",
                receipt.hash.as_ref().expect("Operation hash").value()
            ),
            Some(receipt),
        )
        .map_err(err_into)
    }

    fn get_operation_receipt(&mut self, hash: &str) -> Result<Operation> {
        self.get::<Operation>(format!("/operations/{}", hash))
            .map_err(err_into)?
            .ok_or(Error::OperationNotFound { hash: hash.into() })
    }

    fn get_contract_code(&mut self, address: &str) -> Result<Option<Micheline>> {
        self.get(format!("/context/contracts/{}/code", address))
            .map_err(err_into)
    }

    fn set_contract_code(&mut self, address: &str, code: Micheline) -> Result<()> {
        self.set(format!("/context/contracts/{}/code", address), Some(code))
            .map_err(err_into)
    }

    fn get_contract_storage(&mut self, address: &str) -> Result<Option<Micheline>> {
        self.get(format!("/context/contracts/{}/storage", address))
            .map_err(err_into)
    }

    fn set_contract_storage(&mut self, address: &str, storage: Micheline) -> Result<()> {
        self.set(
            format!("/context/contracts/{}/storage", address),
            Some(storage),
        )
        .map_err(err_into)
    }

    fn add_pending_operation(&mut self, level: i32, operation: SignedOperation) -> Result<()> {
        let hash = operation.hash()?;
        self.set(
            format!(
                "/mempool/{}",
                receipt.hash.as_ref().expect("Operation hash").value()
            ),
            Some(receipt),
        )
        .map_err(err_into)
    }

    fn check_no_pending_changes(&self) -> Result<()> {
        if self.has_pending_changes() {
            Err(layered_store::Error::ContextUnstagedError.into())
        } else {
            Ok(())
        }
    }

    fn commit(&mut self) -> Result<()> {
        LayeredStore::commit(self).map_err(err_into)
    }

    fn rollback(&mut self) {
        LayeredStore::rollback(self)
    }

    fn log(&self, msg: String) {
        LayeredStore::log(&self, msg)
    }
}
