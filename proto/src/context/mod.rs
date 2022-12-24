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
use tezos_michelson::micheline::Micheline;
use tezos_rpc::models::{
    operation::Operation as OperationReceipt,
};

use crate::{
    context::{
        types::{ContextNodeType, TezosAddress},
        head::Head,
        checksum::Checksum
    },
    producer::types::BatchReceipt,
    Result
};

#[macro_export]
macro_rules! assert_no_pending_changes {
    ($ctx: expr) => {
        if $ctx.has_pending_changes() {
            return Err(crate::error::Error::ContextUnstagedError);
        }
    };
}

pub trait Context {
    fn log(&self, msg: String);
    fn has(&self, key: String) -> Result<bool>;
    fn get<V: ContextNodeType>(&mut self, key: String) -> Result<Option<V>> ;
    fn set<V: ContextNodeType>(&mut self, key: String, val: V) -> Result<()>;
    fn persist<V: ContextNodeType>(&mut self, key: String, val: V, rew_lvl: Option<i32>) -> Result<()>;
    fn has_pending_changes(&self) -> bool;
    fn commit(&mut self) -> Result<()>;
    fn rollback(&mut self);
    fn clear(&mut self);

    fn get_checksum(&mut self) -> Result<Checksum> {
        match self.get("/context/checksum".into()) {
            Ok(Some(value)) => Ok(value),
            Ok(None) => Ok(Checksum::default()),
            Err(err) => Err(err)
        }
    }

    fn commit_checksum(&mut self, checksum: Checksum, level: i32) -> Result<()> {
        self.persist("/context/checksum".into(), checksum, Some(level))
    }

    fn get_head(&mut self) -> Result<Head> {
        match self.get("/head".into()) {
            Ok(Some(value)) => Ok(value),
            Ok(None) => Ok(Head::default()),
            Err(err) => Err(err)
        }
    }

    fn commit_head(&mut self, head: Head) -> Result<()> {
        let rew_lvl = head.level - 1;
        self.persist("/head".into(), head, Some(rew_lvl))
    }

    fn rewind(&mut self, ) -> Result<()> {
        todo!("Iterate over /rewind/current and apply reverse updates, change head")
    }

    fn get_balance(&mut self, address: &impl TezosAddress) -> Result<Option<Mutez>> {
        return self.get(format!("/context/contracts/{}/balance", address.to_string()));
    }

    fn set_balance(&mut self, address: &impl TezosAddress, balance: &Mutez) -> Result<()> {
        return self.set(format!("/context/contracts/{}/balance", address.to_string()), balance.to_owned());
    }

    fn get_counter(&mut self, address: &impl TezosAddress) -> Result<Option<Nat>> {
        return self.get(format!("/context/contracts/{}/counter", address.to_string()));
    }

    fn set_counter(&mut self, address: &impl TezosAddress, counter: &Nat) -> Result<()> {
        return self.set(format!("/context/contracts/{}/counter", address.to_string()), counter.to_owned());
    }

    fn get_public_key(&mut self, address: &impl TezosAddress) -> Result<Option<PublicKey>> {
        return self.get(format!("/context/contracts/{}/pubkey", address.to_string()));
    }

    fn set_public_key(&mut self, address: &impl TezosAddress, public_key: &PublicKey) -> Result<()> {
        // NOTE: Underscores are not supported by host
        return self.set(format!("/context/contracts/{}/pubkey", address.to_string()), public_key.to_owned());
    }

    fn has_public_key(&self, address: &impl TezosAddress) -> Result<bool> {
        return self.has(format!("/context/contracts/{}/pubkey", address.to_string()));
    }

    fn commit_operation_receipt(&mut self, level: i32, index: i32, receipt: OperationReceipt) -> Result<()> {
        if let Some(hash) = &receipt.hash {
            self.persist(format!("/blocks/{}/ophashes/{}", level, index), hash.clone(), None)?;
        }
        self.persist(format!("/blocks/{}/operations/{}", level, index), receipt, None)
    }

    fn get_operation_receipt(&mut self, level: i32, index: i32) -> Result<Option<OperationReceipt>> {
        // TODO: support larger files (read loop)
        return self.get(format!("/blocks/{}/operations/{}", level, index));
    }

    fn commit_batch_receipt(&mut self, level: i32, receipt: BatchReceipt) -> Result<()> {
        self.persist(format!("/blocks/{}/hash", level), receipt.hash.clone(), None)?;
        self.persist(format!("/blocks/{}/header", level), receipt, None)
    }

    fn get_batch_receipt(&mut self, level: i32) -> Result<Option<BatchReceipt>> {
        return self.get(format!("/blocks/{}/header", level));
    }

    fn get_contract_code(&mut self, address: &impl TezosAddress) -> Result<Option<Micheline>> {
        self.get(format!("/context/contracts/{}/code", address.to_string()))
    }

    fn set_contract_code(&mut self, address: &impl TezosAddress, code: Micheline) -> Result<()> {
        self.set(format!("/context/contracts/{}/code", address.to_string()), code)
    }

    fn get_contract_storage(&mut self, address: &impl TezosAddress) -> Result<Option<Micheline>> {
        self.get(format!("/context/contracts/{}/storage", address.to_string()))
    }

    fn set_contract_storage(&mut self, address: &impl TezosAddress, storage: Micheline) -> Result<()> {
        self.set(format!("/context/contracts/{}/storage", address.to_string()), storage)
    }
}
