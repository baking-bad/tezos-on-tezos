// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use actix_web::web::Bytes;
use async_trait::async_trait;
use layered_store::{ephemeral::EphemeralCopy, StoreType};
use log::debug;
use std::collections::VecDeque;
use std::sync::Mutex;
use std::{cell::RefCell, sync::Arc};
use tezos_core::types::encoded::{BlockHash, ChainId, Encoded, OperationHash};
use tezos_operation::operations::SignedOperation;
use tezos_proto::{
    batcher::apply_batch,
    context::{head::Head, migrations::run_migrations, TezosContext, TezosEphemeralContext},
    executor::operation::execute_operation,
    validator::operation::{validate_operation, ValidatedOperation},
};
use tezos_rpc::models::operation::Operation;
use tezos_rpc::models::version::{
    AdditionalInfo, CommitInfo, NetworkVersion, Version, VersionInfo,
};
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::{
    rollup::{rpc_helpers::parse_operation, BlockId, RollupClient, TezosHelpers},
    Error, Result,
};

const CHAIN_ID: &str = "NetXP2FfcNxFANL";

pub struct RollupMockClient {
    context: Mutex<RefCell<TezosEphemeralContext>>,
    mempool: Mutex<RefCell<Vec<(OperationHash, SignedOperation)>>>,
    ttl_blocks: Arc<Mutex<VecDeque<BlockHash>>>,
    channels: Arc<Mutex<Vec<Sender<Result<Bytes>>>>>,
}

macro_rules! get_mut {
    ($item: expr) => {
        $item.lock().expect("Failed to acquire lock").get_mut()
    };
}

const MAX_TTL_BLOCKS_COUNT: usize = 60;

impl Default for RollupMockClient {
    fn default() -> Self {
        Self {
            context: Mutex::new(RefCell::new(TezosEphemeralContext::default())),
            mempool: Mutex::new(RefCell::new(Vec::new())),
            ttl_blocks: Arc::new(Mutex::new(VecDeque::with_capacity(MAX_TTL_BLOCKS_COUNT))),
            channels: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl RollupMockClient {
    pub async fn bake(&self) -> Result<()> {
        let head = get_mut!(self.context).get_head()?;
        let res = apply_batch(
            get_mut!(self.context),
            head,
            get_mut!(self.mempool).drain(..).collect(),
            true,
        )?;
        debug!("Baked {}", res);
        Ok(())
    }

    pub fn patch(&self, func: fn(&mut TezosEphemeralContext) -> Result<()>) -> Result<()> {
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

    async fn store_get<T: StoreType>(&self, key: String, block_id: &BlockId) -> Result<T> {
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
        Ok(true)
    }

    async fn inject_batch(&self, _messages: Vec<Vec<u8>>) -> Result<()> {
        unreachable!()
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

#[async_trait]
impl TezosHelpers for RollupMockClient {
    async fn inject_operation(&self, payload: Vec<u8>) -> Result<OperationHash> {
        let (hash, opg) = parse_operation(payload.as_slice())?;
        debug!("Injected {}\n{:#?}", hash.value(), &opg);
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
        let opg = match validate_operation(&mut context, operation, hash, true)? {
            ValidatedOperation::Valid(opg) => opg,
            ValidatedOperation::Invalid(errors) => return Err(errors.into()),
        };
        let receipt = execute_operation(&mut context, &opg)?;
        Ok(receipt)
    }
}
