use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use tezos_core::types::encoded::{ChainId, Encoded, SmartRollupAddress};
use tezos_ctx::ContextNode;

use crate::{
    internal_error,
    rollup::{BlockId, RollupClient},
    Error, Result,
};

#[derive(Deserialize)]
pub struct StateError {
    pub kind: String,
    pub id: String,
    pub block: Option<String>,
    pub msg: Option<String>,
}

impl std::fmt::Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.block, &self.msg) {
            (Some(hash), None) => f.write_fmt(format_args!("[{}] {}", self.id, hash)),
            (None, Some(msg)) => f.write_fmt(format_args!("[{}] {}", self.id, msg)),
            (None, None) => f.write_str(self.id.as_str()),
            _ => unreachable!(),
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum StateResponse {
    Value(String),
    Errors(Vec<StateError>),
}

#[derive(Clone, Debug)]
pub struct RollupRpcClient {
    pub base_url: String,
    client: Client,
    chain_id: Option<ChainId>,
    origination_level: Option<u32>,
}

impl Default for RollupRpcClient {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:8932".into(),
            client: Client::new(),
            origination_level: None,
            chain_id: None,
        }
    }
}

impl RollupRpcClient {
    pub async fn get_rollup_address(&self) -> Result<SmartRollupAddress> {
        let res = self
            .client
            .get(format!("{}/global/smart_rollup_address", self.base_url))
            .send()
            .await?;

        if res.status() == 200 {
            let value: String = res.json().await?;
            Ok(SmartRollupAddress::new(value)?)
        } else {
            Err(Error::RollupClientError {
                status: res.status().as_u16(),
            })
        }
    }

    pub async fn get_tezos_level(&self) -> Result<u32> {
        let res = self
            .client
            .get(format!("{}/global/tezos_level", self.base_url))
            .send()
            .await?;

        if res.status() == 200 {
            let value = res.text().await?;
            Ok(u32::from_str_radix(&value, 10)?)
        } else {
            Err(Error::RollupClientError {
                status: res.status().as_u16(),
            })
        }
    }

    pub async fn get_state_level(&self, block_id: &BlockId) -> Result<u32> {
        let block_id = self.convert_block_id(block_id).await?;
        let res = self
            .client
            .get(format!(
                "{}/global/block/{}/state_current_level",
                self.base_url, block_id
            ))
            .send()
            .await?;

        if res.status() == 200 {
            let value = res.text().await?;
            Ok(u32::from_str_radix(&value, 10)?)
        } else {
            Err(Error::RollupClientError {
                status: res.status().as_u16(),
            })
        }
    }

    async fn convert_block_id(&self, block_id: &BlockId) -> Result<String> {
        let origination_level = self
            .origination_level
            .ok_or(internal_error!(Misc, "Origination level unknown"))?;

        match block_id {
            BlockId::Head => Ok("head".into()),
            BlockId::Genesis => Ok(origination_level.to_string()),
            BlockId::Level(level) => Ok((level + origination_level).to_string()),
            BlockId::Offset(offset) => {
                let state_head = self.get_tezos_level().await?;
                Ok((state_head - offset).to_string())
            }
            BlockId::Hash(hash) => Err(Error::KeyNotFound {
                key: hash.into_string(),
            }),
        }
    }
}

#[async_trait]
impl RollupClient for RollupRpcClient {
    async fn initialize(&mut self) -> Result<()> {
        let address = self.get_rollup_address().await?;
        let payload = address.to_bytes()?;
        self.chain_id = Some(ChainId::from_bytes(&payload.as_slice()[..4])?);

        let state_level = self.get_state_level(&BlockId::Head).await?;
        let head_level: u32 = self
            .get_batch_head(&BlockId::Head)
            .await?
            .level
            .try_into()?;
        self.origination_level = Some(state_level - head_level);
        Ok(())
    }

    async fn get_state_value(&self, key: String, block_id: &BlockId) -> Result<ContextNode> {
        let block_id = self.convert_block_id(block_id).await?;
        let res = self
            .client
            .get(format!(
                "{}/global/block/{}/durable/wasm_2_0_0/value?key={}",
                self.base_url, block_id, key
            ))
            .send()
            .await?;

        if res.status() == 200 || res.status() == 500 {
            let content: Option<StateResponse> = res.json().await?;
            match content {
                Some(StateResponse::Value(value)) => {
                    let payload = hex::decode(value)?;
                    Ok(ContextNode::from_vec(payload)?)
                }
                Some(StateResponse::Errors(errors)) => {
                    let message = errors.first().unwrap().to_string();
                    Err(Error::DurableStorageError { message })
                }
                None => Err(Error::KeyNotFound { key }),
            }
        } else {
            Err(Error::RollupClientError {
                status: res.status().as_u16(),
            })
        }
    }

    async fn get_chain_id(&self) -> Result<ChainId> {
        self.chain_id
            .clone()
            .ok_or(internal_error!(Misc, "Chain ID unknown"))
    }

    async fn is_chain_synced(&self) -> Result<bool> {
        let tezos_level = self.get_tezos_level().await?;
        let state_level = self.get_state_level(&BlockId::Head).await?;
        Ok(state_level == tezos_level)
    }

    async fn inject_batch(&self, messages: Vec<Vec<u8>>) -> Result<()> {
        let res = self
            .client
            .post(format!("{}/local/batcher/injection", self.base_url))
            .json(&messages)
            .send()
            .await?;

        if res.status() == 200 {
            Ok(())
        } else {
            Err(Error::RollupClientError {
                status: res.status().as_u16(),
            })
        }
    }
}
