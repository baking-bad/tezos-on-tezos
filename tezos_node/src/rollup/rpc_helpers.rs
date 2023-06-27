// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use actix_web::rt::task;
use async_trait::async_trait;
use layered_store::LayeredStore;
use tezos_core::types::encoded::{Encoded, OperationHash, Signature};
use tezos_operation::operations::{SignedOperation, UnsignedOperation};
use tezos_proto::{
    executor::operation::execute_operation,
    validator::operation::{validate_operation, ValidatedOperation},
};
use tezos_rpc::models::operation::Operation;

use crate::{
    rollup::{
        block_id::BlockId, rpc_backend::RpcBackend, rpc_client::RollupRpcClient, RollupClient,
        TezosHelpers,
    },
    Error, Result,
};

// TODO: reuse from kernel?
pub fn parse_operation(payload: &[u8]) -> Result<(OperationHash, SignedOperation)> {
    const SIGNATURE_SIZE: usize = 64;
    if payload.len() < SIGNATURE_SIZE {
        return Err(Error::InvalidArguments {
            message: format!("Payload too short"),
        });
    }

    let operation =
        UnsignedOperation::from_forged_bytes(&payload[..payload.len() - SIGNATURE_SIZE])?;
    let signature = Signature::from_bytes(&payload[payload.len() - SIGNATURE_SIZE..])?;
    let opg = SignedOperation::from(operation, signature);
    let hash = SignedOperation::operation_hash(payload)?;
    Ok((hash, opg))
}

#[async_trait]
impl TezosHelpers for RollupRpcClient {
    async fn inject_operation(&self, payload: Vec<u8>) -> Result<OperationHash> {
        let (hash, _) = parse_operation(payload.as_slice())?;
        let chain_id = self.get_chain_id().await?;
        let message = [chain_id.to_bytes()?, payload].concat();
        self.inject_batch(vec![message]).await?;
        Ok(hash)
    }

    async fn simulate_operation(
        &self,
        block_id: &BlockId,
        operation: SignedOperation,
    ) -> Result<Operation> {
        let state_level = self.get_state_level(block_id).await?;
        let base_url = self.base_url.clone();

        task::spawn_blocking(move || -> Result<Operation> {
            let mut context = LayeredStore::new(RpcBackend::new(base_url, state_level));
            let hash = operation.hash()?;
            let opg = match validate_operation(&mut context, operation, hash, true)? {
                ValidatedOperation::Valid(opg) => opg,
                ValidatedOperation::Invalid(errors) => return Err(errors.into()),
            };
            let receipt = execute_operation(&mut context, &opg)?;
            Ok(receipt)
        })
        .await?
    }
}
