use async_trait::async_trait;
use layered_store::StoreType;
use log::debug;
use reqwest::Client;
use serde::Deserialize;
use tezos_core::types::encoded::{ChainId, Encoded, SmartRollupAddress};
use tezos_proto::context::TezosStoreType;
use tezos_rpc::models::version::{
    AdditionalInfo, CommitInfo, NetworkVersion, Version, VersionInfo,
};

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

#[derive(Deserialize)]
#[serde(untagged)]
pub enum LevelResponse {
    Value(u32),
    Errors(Vec<StateError>),
}

#[derive(Clone, Debug)]
pub struct RollupRpcClient {
    pub base_url: String,
    client: Client,
    chain_id: Option<ChainId>,
    origination_level: Option<u32>,
}

impl RollupRpcClient {
    pub fn new(endpoint: &str) -> Self {
        Self {
            base_url: endpoint.into(),
            client: Client::new(),
            origination_level: None,
            chain_id: None,
        }
    }

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
            let content: LevelResponse = res.json().await?;
            match content {
                LevelResponse::Value(value) => Ok(value),
                LevelResponse::Errors(errors) => {
                    let message = errors.first().unwrap().to_string();
                    Err(Error::RollupInternalError { message })
                }
            }
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
            let content: LevelResponse = res.json().await?;
            match content {
                LevelResponse::Value(value) => Ok(value),
                LevelResponse::Errors(errors) => {
                    let message = errors.first().unwrap().to_string();
                    Err(Error::RollupInternalError { message })
                }
            }
        } else {
            Err(Error::RollupClientError {
                status: res.status().as_u16(),
            })
        }
    }

    async fn convert_block_id(&self, block_id: &BlockId) -> Result<String> {
        match block_id {
            BlockId::Head => Ok("head".into()),
            BlockId::Level(level) => {
                let origination_level = self
                    .origination_level
                    .ok_or(internal_error!(Misc, "Origination level yet unknown"))?;
                Ok((level + origination_level).to_string())
            }
            BlockId::Offset(offset) => {
                let state_head = self.get_tezos_level().await?;
                Ok((state_head - offset).to_string())
            }
            BlockId::Hash(hash) => {
                let level = self.get_batch_level(hash).await?;
                let origination_level = self
                    .origination_level
                    .ok_or(internal_error!(Misc, "Origination level yet unknown"))?;
                Ok(((level as u32) + origination_level).to_string())
            }
        }
    }
}

#[async_trait]
impl RollupClient for RollupRpcClient {
    async fn initialize(&mut self) -> Result<()> {
        let address = self.get_rollup_address().await?;
        debug!("Rollup address: {}", address.value());

        let payload = address.to_bytes()?;
        self.chain_id = Some(ChainId::from_bytes(&payload.as_slice()[..4])?);
        debug!("Chain ID: {}", self.chain_id.as_ref().unwrap().value());

        let state_level = self.get_state_level(&BlockId::Head).await?;
        debug!("PVM state level: {}", state_level);

        let head_level: u32 = self
            .get_batch_head(&BlockId::Head)
            .await?
            .level
            .try_into()?;
        debug!("Chain head level: {}", head_level);

        self.origination_level = Some(state_level - head_level);
        debug!(
            "Rollup origination level: {}",
            self.origination_level.as_ref().unwrap()
        );
        Ok(())
    }

    async fn get_state_value(&self, key: String, block_id: &BlockId) -> Result<TezosStoreType> {
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
                    Ok(TezosStoreType::from_vec(payload)?)
                }
                Some(StateResponse::Errors(errors)) => {
                    let message = errors.first().unwrap().to_string();
                    Err(Error::RollupInternalError { message })
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

    async fn get_version(&self) -> Result<VersionInfo> {
        Ok(VersionInfo {
            version: Version {
                major: 0,
                minor: 0,
                additional_info: AdditionalInfo::Dev,
            },
            network_version: NetworkVersion {
                chain_name: "TEZOS-ROLLUP-2023-02-08T00:00:00.000Z".into(),
                distributed_db_version: 0,
                p2p_version: 0,
            },
            commit_info: CommitInfo {
                commit_hash: "00000000".into(),
                commit_date: "2023-02-08 00:00:00 +0000".into(),
            },
        })
    }

    async fn is_chain_synced(&self) -> Result<bool> {
        let tezos_level = self.get_tezos_level().await?;
        let state_level = self.get_state_level(&BlockId::Head).await?;
        Ok(state_level == tezos_level)
    }

    async fn inject_batch(&self, messages: Vec<Vec<u8>>) -> Result<()> {
        let messages: Vec<String> = messages.into_iter().map(|msg| hex::encode(msg)).collect();

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
