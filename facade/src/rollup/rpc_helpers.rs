use async_trait::async_trait;
use actix_web::rt::task;
use tezos_rpc::models::operation::Operation;
use tezos_operation::operations::SignedOperation;
use tezos_l2::{
    validator::operation::validate_operation,
    executor::operation::execute_operation,
};

use crate::{
    Result,
    rollup::{
        rpc_context::RpcContext,
        rpc_client::RollupRpcClient,
        block_id::BlockId,
        TezosHelpers,
    }
};

#[async_trait]
impl TezosHelpers for RollupRpcClient {
    async fn simulate_operation(&self, block_id: &BlockId, operation: SignedOperation) -> Result<Operation> {
        let state_level = self.get_state_level(block_id).await?;
        let base_url = self.base_url.clone();

        task::spawn_blocking(move || -> Result<Operation> {
            let mut context = RpcContext::new(base_url, state_level);
            let hash = operation.hash()?;
            let opg = validate_operation(&mut context, operation, hash, true)?;
            let receipt = execute_operation(&mut context, &opg)?;
            Ok(receipt)
        }).await?
    }
}