// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use actix_web::web::{Bytes, Data};
use async_trait::async_trait;
use chrono::Utc;
use layered_store::StoreType;
use log::debug;
use reqwest::Client;
use serde::Deserialize;
use tezos_core::types::encoded::{BlockHash, ChainId, Encoded, SmartRollupAddress};
use tezos_rpc::models::{
    block::FullHeader,
    version::{AdditionalInfo, CommitInfo, NetworkVersion, Version, VersionInfo},
};
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    time::{sleep, Duration},
};

use crate::{
    internal_error,
    rollup::{BlockId, RollupClient},
    services::blocks::HeaderShell,
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
    ttl_blocks: Arc<Mutex<VecDeque<BlockHash>>>,
    channels: Arc<Mutex<Vec<Sender<Result<Bytes>>>>>,
}

const MAX_TTL_BLOCKS_COUNT: i32 = 60;

impl RollupRpcClient {
    pub fn new(endpoint: &str) -> Self {
        Self {
            base_url: endpoint.into(),
            client: Client::new(),
            origination_level: None,
            chain_id: None,
            ttl_blocks: Arc::new(Mutex::new(VecDeque::with_capacity(
                MAX_TTL_BLOCKS_COUNT as usize,
            ))),
            channels: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // FIXME: too much copypaste in the following several methods
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

    // Code duplication
    async fn store_get<T: StoreType>(&self, key: String, block_id: &BlockId) -> Result<T> {
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
                    Ok(StoreType::from_bytes(payload.as_slice())?)
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

    fn create_channel(&self) -> Result<Receiver<Result<Bytes>>> {
        const LONG_POLL_CHANNEL_SIZE: usize = 1;
        let (tx, rx) = channel::<Result<Bytes>>(LONG_POLL_CHANNEL_SIZE);
        let mut channels = self.channels.lock().unwrap();
        channels.push(tx);
        Ok(rx)
    }

    fn get_ttl_blocks(&self) -> Result<Arc<Mutex<VecDeque<BlockHash>>>> {
        Ok(Arc::clone(&self.ttl_blocks))
    }

    async fn broadcast_to_channels(&self, data: Bytes) -> Result<()> {
        let mut channels = self.channels.lock().unwrap();
        let mut i = 0;
        while i < channels.len() {
            if channels[i].is_closed() {
                channels.remove(i);
                continue;
            }

            let value = data.clone();
            if let Err(_) = channels[i].try_send(Ok(value)) {
                channels.remove(i);
                continue;
            }

            i += 1;
        }

        Ok(())
    }

    fn channels_count(&self) -> usize {
        let channels_ptr = self.channels.lock().unwrap();
        channels_ptr.len()
    }
}

pub fn run_block_updater<T: RollupClient + 'static>(client: &Data<T>) -> () {
    const BLOCK_INTERVAL_SEC: i64 = 8;
    const BLOCK_DELAY_SEC: i64 = 3;

    let client = client.clone();
    tokio::spawn(async move {
        // TODO: wait chain sync?
        let mut curr_level = 0;

        loop {
            let timestamp = Utc::now().timestamp();
            let head = client.get_batch_receipt(&BlockId::Head).await.unwrap();
            let head_timestamp = head.header.timestamp;

            if curr_level == head.header.level {
                sleep(Duration::from_secs(BLOCK_DELAY_SEC as u64)).await;
                continue;
            }

            curr_level = std::cmp::max(curr_level, head.header.level - MAX_TTL_BLOCKS_COUNT);

            while curr_level < head.header.level {
                let batch_receipt = client
                    .get_batch_receipt(&BlockId::Level(curr_level.try_into().unwrap()))
                    .await
                    .unwrap();

                {
                    let ttl_blocks_ptr = client.get_ttl_blocks().unwrap();
                    let mut ttl_blocks: std::sync::MutexGuard<'_, VecDeque<BlockHash>> =
                        ttl_blocks_ptr.lock().unwrap();

                    if ttl_blocks.len() == MAX_TTL_BLOCKS_COUNT as usize {
                        ttl_blocks.pop_front();
                    }
                    ttl_blocks.push_back(batch_receipt.hash.clone());
                }

                curr_level += 1;

                if client.channels_count() == 0 {
                    continue;
                }

                let full_header: FullHeader = batch_receipt.into();

                let header = HeaderShell {
                    hash: Some(full_header.hash),
                    level: full_header.level,
                    proto: full_header.proto,
                    predecessor: full_header.predecessor,
                    timestamp: full_header.timestamp,
                    validation_pass: full_header.validation_pass,
                    operations_hash: full_header.operations_hash,
                    fitness: full_header.fitness,
                    context: full_header.context,
                    protocol_data: Some("".to_string()),
                };

                let header_json = serde_json::to_string(&header).unwrap();
                let header_bytes = Bytes::from(header_json);

                if let Err(_) = client.broadcast_to_channels(header_bytes).await {
                    debug!("Error while broadcast header to long polls clients");
                }
            }

            let next_block_timestamp = head_timestamp + BLOCK_INTERVAL_SEC + BLOCK_DELAY_SEC;
            let waiting_interval = std::cmp::max(next_block_timestamp - timestamp, BLOCK_DELAY_SEC);
            sleep(Duration::from_secs(waiting_interval as u64)).await;
        }
    });
}
