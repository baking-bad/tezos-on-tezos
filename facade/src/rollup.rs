use async_trait::async_trait;
use ibig::IBig;
use reqwest::Client;
use tezos_rpc::models::{
    block::{Block, FullHeader, Metadata as BlockMetadata},
    operation::Operation, 
    contract::{ContractScript, ContractInfo, ContractEntrypoints},
    constants::Constants,
};
use tezos_core::types::{
    encoded::{OperationHash, BlockHash, Address, ImplicitAddress, ChainId},
};
use tezos_michelson::micheline::Micheline;
use context::ContextNode;

use crate::{
    Result,
    Error,
    provider::{RPCProvider, BlockId},
    internal_error
};

pub struct RollupRpcClient {
    client: Client,
    uri: String
}

impl RollupRpcClient {
    pub fn default() -> Self {
        Self { client: Client::default(), uri: "http://localhost:8932".into() }
    }

    async fn get(&self, path: String) -> Result<ContextNode> {
        let res = self.client.get(
            format!("{}/global/durable{}/@/contents/0", self.uri, path)
        ).send().await?;
        let body = res.bytes().await?;
        let node = ContextNode::from_vec(body.to_vec())?;
        Ok(node)
    }
}

#[async_trait]
impl RPCProvider for RollupRpcClient {
    async fn get_block_hash(&self, block_id: BlockId) -> Result<BlockHash> {
        let node = self.get(format!("/blocks/{}/hash", block_id)).await?;
        match node {
            ContextNode::BlockHash(value) => Ok(value),
            _ => Err(internal_error!(Parsing, "Unexpected node type"))
        }
    }
}