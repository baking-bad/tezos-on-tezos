use reqwest::Client;
use serde::Deserialize;
use async_trait::async_trait;
use context::ContextNode;
use tezos_core::types::encoded::Encoded;

use crate::{Result, Error, rollup::{RollupClient, BlockId}};

#[derive(Deserialize)]
pub struct StateError {
    pub kind: String,
    pub id: String,
    pub block: Option<String>,
    pub msg: Option<String>
}

impl std::fmt::Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.block, &self.msg) {
            (Some(hash), None) => f.write_fmt(format_args!("[{}] {}", self.id, hash)),
            (None, Some(msg)) => f.write_fmt(format_args!("[{}] {}", self.id, msg)),
            (None, None) => f.write_str(self.id.as_str()),
            _ => unreachable!()
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum StateResponse {
    Value(String),
    Errors(Vec<StateError>)
}

pub struct RPCClient {
    base_url: String,
    client: Client,
    origination_level: i32
}

impl RPCClient {
    pub fn default() -> Self {
        Self {
            base_url: "http://localhost:8932".into(),
            client: Client::new(),
            origination_level: 0,
        }
    }
}

#[async_trait]
impl RollupClient for RPCClient {
    async fn get_state_value(&self, key: String, block_id: &BlockId) -> Result<ContextNode> {
        let block_id: String = match block_id {
            BlockId::Genesis => "genesis".into(),
            BlockId::Head => "head".into(),
            BlockId::Level(level) => (level + self.origination_level).to_string(),
            BlockId::Hash(hash) => hash.into_string(),
        };

        let res = self.client
            .get(format!("{}/global/block/{}/durable/wasm_2_0_0/value?key={}", self.base_url, block_id, key))
            .send()
            .await?;

        if res.status() == 200 || res.status() == 500 {
            let content: Option<StateResponse> = res.json().await?;
            match content {
                Some(StateResponse::Value(value)) => {
                    let payload = hex::decode(value)?;
                    Ok(ContextNode::from_vec(payload)?)
                },
                Some(StateResponse::Errors(errors)) => {
                    let message = errors.first().unwrap().to_string();
                    Err(Error::DurableStorageError { message })
                },
                None => Err(Error::KeyNotFound { key })
            }
        } else {
            Err(Error::RollupClientError { status: res.status().as_u16() })
        }
    }
}