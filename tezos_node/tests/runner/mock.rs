use async_trait::async_trait;
use context::{ContextNode, Head, EphemeralContext, GenericContext, migrations::run_migrations};
use tezos_l2::batcher::apply_batch;
use tezos_core::types::encoded::ChainId;
use tezos_operation::operations::SignedOperation;
use tezos_rpc::models::operation::Operation;
use std::sync::Mutex;
use std::cell::RefCell;

use tezos_node::{
    Result,
    Error,
    rollup::{RollupClient, TezosHelpers, BlockId}
};

pub struct RollupMockClient {
    context: Mutex<RefCell<EphemeralContext>>
}

impl Default for RollupMockClient {
    fn default() -> Self {
        Self {
            context: Mutex::new(RefCell::new(EphemeralContext::new()))
        }
    }
}

impl RollupMockClient {
    pub async fn initialize(&self) -> Result<()> {
        let head = Head::default();
        run_migrations(self.context
            .lock()
            .expect("Failed to acquire lock")
            .get_mut(), &head)?;
        apply_batch(self.context
            .lock()
            .expect("Failed to acquire lock")
            .get_mut(), head, vec![], false)?;
        Ok(())
    }
}

#[async_trait]
impl RollupClient for RollupMockClient {
    async fn get_state_value(&self, key: String, block_id: &BlockId) -> Result<ContextNode> {
        match &block_id {
            BlockId::Head => {},
            _ => unimplemented!("Only head supported in mockup mode")
        }
        self.context
            .lock()
            .expect("Failed to acquire lock")
            .borrow_mut()
            .get(key.clone())?
            .ok_or(Error::KeyNotFound { key })
    }

    async fn get_chain_id(&self) -> Result<ChainId> {
        todo!()
    }

    async fn inject_batch(&self, messages: Vec<Vec<u8>>) -> Result<()> {
        todo!()
    }
}

#[async_trait]
impl TezosHelpers for RollupMockClient {
    async fn simulate_operation(&self, block_id: &BlockId, operation: SignedOperation) -> Result<Operation> {
        todo!()
    }
}
