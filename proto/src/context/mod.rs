pub mod types;
pub mod ephemeral;
pub mod head;
pub mod checksum;
pub mod migrations;

use tezos_core::types::{
    encoded::PublicKey,
    mutez::Mutez,
    number::Nat
};
use tezos_rpc::models::{
    operation::Operation as OperationReceipt,
};

use crate::{context::{
    types::{ContextNodeType, TezosAddress},
    head::Head,
    checksum::Checksum
}, producer::types::BatchReceipt};
use crate::error::Result;

pub trait Context {
    fn has(&self, key: String) -> Result<bool>;
    fn get<V: ContextNodeType>(&mut self, key: String, max_bytes: usize) -> Result<Option<V>> ;
    fn set<V: ContextNodeType>(&mut self, key: String, val: V) -> Result<()>;
    fn persist<V: ContextNodeType>(&mut self, key: String, val: V) -> Result<()>;
    fn has_pending_changes(&self) -> bool;
    fn commit(&mut self) -> Result<()>;
    fn rollback(&mut self);
    fn clear(&mut self);

    fn get_checksum(&mut self) -> Result<Checksum> {
        match self.get("/context/checksum".into(), 32) {
            Ok(Some(value)) => Ok(value),
            Ok(None) => Ok(Checksum::default()),
            Err(err) => Err(err)
        }
    }

    fn get_head(&mut self) -> Result<Head> {
        match self.get(format!("/kernel/head"), 36) {
            Ok(Some(value)) => Ok(value),
            Ok(None) => Ok(Head::default()),
            Err(err) => Err(err)
        }
    }

    fn get_balance(&mut self, address: &impl TezosAddress) -> Result<Option<Mutez>> {
        return self.get(format!("/context/contracts/{}/balance", address.to_string()), 64);
    }

    fn set_balance(&mut self, address: &impl TezosAddress, balance: &Mutez) -> Result<()> {
        return self.set(format!("/context/contracts/{}/balance", address.to_string()), balance.to_owned());
    }

    fn get_counter(&mut self, address: &impl TezosAddress) -> Result<Option<Nat>> {
        return self.get(format!("/context/contracts/{}/counter", address.to_string()), 64);
    }

    fn set_counter(&mut self, address: &impl TezosAddress, counter: &Nat) -> Result<()> {
        return self.set(format!("/context/contracts/{}/counter", address.to_string()), counter.to_owned());
    }

    fn get_public_key(&mut self, address: &impl TezosAddress) -> Result<Option<PublicKey>> {
        return self.get(format!("/context/contracts/{}/pubkey", address.to_string()), 33);
    }

    fn set_public_key(&mut self, address: &impl TezosAddress, public_key: &PublicKey) -> Result<()> {
        // FIXME: Underscores are not supported
        return self.set(format!("/context/contracts/{}/pubkey", address.to_string()), public_key.to_owned());
    }

    fn has_public_key(&self, address: &impl TezosAddress) -> Result<bool> {
        return self.has(format!("/context/contracts/{}/pubkey", address.to_string()));
    }

    fn store_operation_receipt(&mut self, level: &i32, index: &i32, receipt: OperationReceipt) -> Result<()> {
        if let Some(hash) = &receipt.hash {
            self.set(format!("/context/blocks/{}/operation_hashes/{}", level, index), hash.clone())?;
        }
        self.set(format!("/context/blocks/{}/operations/{}", level, index), receipt)
    }

    fn get_operation_receipt(&mut self, level: &i32, index: &i32) -> Result<Option<OperationReceipt>> {
        // TODO: support larger files (read loop)
        return self.get(format!("/context/blocks/{}/operations/{}", level, index), 2048);
    }

    fn store_batch_receipt(&mut self, level: i32, receipt: BatchReceipt) -> Result<()> {
        // TODO: store hash alias
        self.set(format!("/context/blocks/{}/hash", level), receipt.hash.clone())?;
        self.set(format!("/context/blocks/{}/header", level), receipt)
    }

    fn get_batch_receipt(&mut self, level: i32) -> Result<Option<BatchReceipt>> {
        return self.get(format!("/context/blocks/{}/header", level), 1024);
    }
}
