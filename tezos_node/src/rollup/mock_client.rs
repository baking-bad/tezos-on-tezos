use async_trait::async_trait;
use context::{
    migrations::run_migrations, ContextNode, EphemeralContext, ExecutorContext, GenericContext,
    Head,
};
use std::cell::RefCell;
use std::sync::Mutex;
use tezos_core::types::encoded::{ChainId, OperationHash};
use tezos_l2::{
    batcher::apply_batch, executor::operation::execute_operation,
    validator::operation::validate_operation,
};
use tezos_operation::operations::SignedOperation;
use tezos_rpc::models::operation::Operation;

use crate::{
    rollup::{rpc_helpers::parse_operation, BlockId, RollupClient, TezosHelpers},
    Error, Result,
};

const CHAIN_ID: &str = "NetXP2FfcNxFANL";

pub struct RollupMockClient {
    context: Mutex<RefCell<EphemeralContext>>,
    mempool: Mutex<RefCell<Vec<(OperationHash, SignedOperation)>>>,
}

macro_rules! get_mut {
    ($item: expr) => {
        $item.lock().expect("Failed to acquire lock").get_mut()
    };
}

impl Default for RollupMockClient {
    fn default() -> Self {
        Self {
            context: Mutex::new(RefCell::new(EphemeralContext::new())),
            mempool: Mutex::new(RefCell::new(Vec::new())),
        }
    }
}

impl RollupMockClient {
    pub async fn bake(&self) -> Result<()> {
        let head = get_mut!(self.context).get_head()?;
        apply_batch(
            get_mut!(self.context),
            head,
            get_mut!(self.mempool).drain(..).collect(),
            true,
        )?;
        Ok(())
    }

    pub fn patch(&self, func: fn(&mut EphemeralContext) -> Result<()>) -> Result<()> {
        func(get_mut!(self.context))
    }
}

#[async_trait]
impl RollupClient for RollupMockClient {
    async fn initialize(&mut self) -> Result<()> {
        let head = Head::default();
        run_migrations(get_mut!(self.context), &head)?;
        apply_batch(get_mut!(self.context), head, vec![], false)?;
        Ok(())
    }

    async fn get_state_value(&self, key: String, block_id: &BlockId) -> Result<ContextNode> {
        match &block_id {
            BlockId::Head => {}
            _ => unimplemented!("Can only access state at head level in the mockup mode"),
        };
        get_mut!(self.context)
            .get(key.clone())?
            .ok_or(Error::KeyNotFound { key })
    }

    async fn get_chain_id(&self) -> Result<ChainId> {
        Ok(CHAIN_ID.try_into()?)
    }

    async fn is_chain_synced(&self) -> Result<bool> {
        Ok(true)
    }

    async fn inject_batch(&self, _messages: Vec<Vec<u8>>) -> Result<()> {
        unreachable!()
    }
}

#[async_trait]
impl TezosHelpers for RollupMockClient {
    async fn inject_operation(&self, payload: Vec<u8>) -> Result<OperationHash> {
        let (hash, opg) = parse_operation(payload.as_slice())?;
        get_mut!(self.mempool).push((hash.clone(), opg));
        Ok(hash)
    }

    async fn simulate_operation(
        &self,
        block_id: &BlockId,
        operation: SignedOperation,
    ) -> Result<Operation> {
        match &block_id {
            BlockId::Head => {}
            _ => unimplemented!("Can only access state at head level in the mockup mode"),
        };
        let mut context = get_mut!(self.context).spawn();
        let hash = operation.hash()?;
        let opg = validate_operation(&mut context, operation, hash, true)?;
        let receipt = execute_operation(&mut context, &opg)?;
        Ok(receipt)
    }
}
