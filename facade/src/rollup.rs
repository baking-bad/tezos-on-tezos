pub mod rpc_client;
pub mod rpc_context;
pub mod rpc_helpers;
pub mod facade;
pub mod block_id;
pub mod mock_node;

use async_trait::async_trait;
use context::{BatchReceipt, ContextNode, Head};
use serde::Serialize;
use tezos_core::types::encoded::{
    Address, BlockHash, ChainId, ContractAddress, Encoded, ImplicitAddress, OperationHash,
    ProtocolHash, PublicKey, ScriptExprHash,
};
use tezos_core::types::{mutez::Mutez, number::Nat};
use tezos_michelson::micheline::Micheline;
use tezos_operation::operations::SignedOperation;
use tezos_rpc::models::{
    block::{Block, FullHeader, Metadata},
    contract::{ContractEntrypoints, ContractInfo, ContractScript},
    operation::Operation,
};

use crate::Result;
pub use block_id::BlockId;

#[async_trait]
pub trait RollupClient {
    async fn get_state_value(&self, key: String, block_id: &BlockId) -> Result<ContextNode>;
    async fn get_chain_id(&self) -> Result<ChainId>;
    async fn inject_batch(&self, messages: Vec<Vec<u8>>) -> Result<()>;

    async fn get_batch_head(&self, block_id: &BlockId) -> Result<Head> {
        let head: Head = self
            .get_state_value("/head".into(), block_id)
            .await?
            .try_into()?;
        Ok(head)
    }

    async fn get_batch_receipt(&self, block_id: &BlockId) -> Result<BatchReceipt> {
        let hash = match block_id {
            BlockId::Hash(hash) => hash.clone(),
            _ => self.get_batch_head(block_id).await?.hash,
        };
        let receipt: BatchReceipt = self
            .get_state_value(format!("/batches/{}", hash.value()), &BlockId::Head)
            .await?
            .try_into()?;
        Ok(receipt)
    }

    async fn get_operation_receipt(&self, hash: &OperationHash) -> Result<Operation> {
        let receipt: Operation = self
            .get_state_value(format!("/operations/{}", hash.value()), &BlockId::Head)
            .await?
            .try_into()?;
        Ok(receipt)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BlockProtocols {
    pub protocol: ProtocolHash,
    pub next_protocol: ProtocolHash,
}

#[async_trait]
pub trait TezosFacade {
    async fn get_block(&self, block_id: &BlockId) -> Result<Block>;
    async fn get_block_hash(&self, block_id: &BlockId) -> Result<BlockHash>;
    async fn get_block_header(&self, block_id: &BlockId) -> Result<FullHeader>;
    async fn get_block_metadata(&self, block_id: &BlockId) -> Result<Metadata>;
    async fn get_block_protocols(&self, block_id: &BlockId) -> Result<BlockProtocols>;
    async fn get_live_blocks(&self, block_id: &BlockId) -> Result<Vec<BlockHash>>;
    async fn get_contract(&self, block_id: &BlockId, address: &Address) -> Result<ContractInfo>;
    async fn get_contract_balance(&self, block_id: &BlockId, address: &Address) -> Result<Mutez>;
    async fn get_contract_counter(
        &self,
        block_id: &BlockId,
        address: &ImplicitAddress,
    ) -> Result<Nat>;
    async fn get_contract_public_key(
        &self,
        block_id: &BlockId,
        address: &ImplicitAddress,
    ) -> Result<PublicKey>;
    async fn get_contract_code(
        &self,
        block_id: &BlockId,
        address: &ContractAddress,
    ) -> Result<Micheline>;
    async fn get_contract_storage(
        &self,
        block_id: &BlockId,
        address: &ContractAddress,
    ) -> Result<Micheline>;
    async fn get_contract_script(
        &self,
        block_id: &BlockId,
        address: &ContractAddress,
    ) -> Result<ContractScript>;
    async fn get_contract_entrypoints(
        &self,
        block_id: &BlockId,
        address: &ContractAddress,
    ) -> Result<ContractEntrypoints>;
    async fn get_big_map_value(
        &self,
        block_id: &BlockId,
        big_map_id: i64,
        key_hash: &ScriptExprHash,
    ) -> Result<Micheline>;
    async fn get_operation_hash(
        &self,
        block_id: &BlockId,
        pass: i32,
        index: i32,
    ) -> Result<OperationHash>;
    async fn get_operation_hash_list(
        &self,
        block_id: &BlockId,
        pass: i32,
    ) -> Result<Vec<OperationHash>>;
    async fn get_operation_hash_list_list(
        &self,
        block_id: &BlockId,
    ) -> Result<Vec<Vec<OperationHash>>>;
    async fn get_operation(&self, block_id: &BlockId, pass: i32, index: i32) -> Result<Operation>;
    async fn get_operation_list(&self, block_id: &BlockId, pass: i32) -> Result<Vec<Operation>>;
    async fn get_operation_list_list(&self, block_id: &BlockId) -> Result<Vec<Vec<Operation>>>;
    async fn inject_operation(&self, payload: Vec<u8>) -> Result<OperationHash>;
}

#[async_trait]
pub trait TezosHelpers {
    async fn simulate_operation(&self, block_id: &BlockId, operation: SignedOperation) -> Result<Operation>;
}
