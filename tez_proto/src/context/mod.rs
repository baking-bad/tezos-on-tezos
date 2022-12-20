pub mod types;
pub mod ephemeral;
pub mod head;
pub mod checksum;

use tezos_core::types::{
    encoded::{PublicKey, ContextHash, Encoded},
    mutez::Mutez,
    number::Nat
};
use tezos_rpc::models::{
    operation::Operation as OperationReceipt,
    block::FullHeader as BlockHeader,
    block::Metadata as BlockMetadata
};

use crate::context::{
    types::{ContextNodeType, TezosAddress},
    head::Head,
    checksum::Checksum
};
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
        match self.get("/kernel/checksum".into(), 32) {
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

    fn set_head(&mut self, head: &Head) -> Result<()> {
        return self.set(format!("/kernel/head"), head.to_owned());
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

    fn store_operation_receipt(&mut self, level: &i32, index: &i32, receipt: &OperationReceipt) -> Result<()> {
        return self.set(format!("/context/blocks/{}/operations/{}", level, index), receipt.to_owned());
    }

    fn get_operation_receipt(&mut self, level: &i32, index: &i32) -> Result<Option<OperationReceipt>> {
        // TODO: support larger files (read loop)
        return self.get(format!("/context/blocks/{}/operations/{}", level, index), 2048);
    }

    fn store_block_header(&mut self, level: i32, header: &BlockHeader) -> Result<()> {
        return self.set(format!("/context/blocks/{}/header", level), header.to_owned());
    }

    fn get_block_header(&mut self, level: i32) -> Result<Option<BlockHeader>> {
        return self.get(format!("/context/blocks/{}/header", level), 1024);
    }

    fn store_block_metadata(&mut self, level: i32, metadata: &BlockMetadata) -> Result<()> {
        return self.set(format!("/context/blocks/{}/metadata", level), metadata.to_owned());
    }

    fn get_block_metadata(&mut self, level: i32) -> Result<Option<BlockMetadata>> {
        return self.get(format!("/context/blocks/{}/metadata", level), 1024);
    }
}
