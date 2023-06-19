pub mod head;
pub mod batch;
pub mod migrations;
pub mod codec;
pub mod store;

use tezos_core::types::{
    encoded::PublicKey,
    mutez::Mutez,
    number::Nat,
};
use tezos_michelson::micheline::Micheline;
use tezos_rpc::models::operation::Operation;
use layered_store::EphemeralStore;

use crate::{
    context::{
        head::Head,
        batch::BatchReceipt,
        store::TezosStore,
        codec::TezosStoreType,
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

pub type TezosEphemeralContext = TezosStore<EphemeralStore<TezosStoreType>>;

impl TezosEphemeralContext {
    pub fn new() -> Self {
        Self(EphemeralStore::<TezosStoreType>::new())
    }
}