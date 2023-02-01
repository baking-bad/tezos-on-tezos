use actix_web::rt::task;
use async_trait::async_trait;
use tezos_l2::{executor::operation::execute_operation, validator::operation::validate_operation};
use tezos_operation::operations::SignedOperation;
use tezos_rpc::models::operation::Operation;

use crate::{
    rollup::{
        block_id::BlockId, rpc_client::RollupRpcClient, rpc_context::RpcContext, TezosHelpers,
    },
    Result,
};

#[async_trait]
impl TezosHelpers for RollupRpcClient {
    async fn simulate_operation(
        &self,
        block_id: &BlockId,
        operation: SignedOperation,
    ) -> Result<Operation> {
        let state_level = self.get_state_level(block_id).await?;
        let base_url = self.base_url.clone();

        task::spawn_blocking(move || -> Result<Operation> {
            let mut context = RpcContext::new(base_url, state_level);
            let hash = operation.hash()?;
            let opg = validate_operation(&mut context, operation, hash, true)?;
            let receipt = execute_operation(&mut context, &opg)?;
            Ok(receipt)
        })
        .await?
    }
}
