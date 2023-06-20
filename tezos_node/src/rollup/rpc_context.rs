use layered_store::{error::err_into, EphemeralStore, LayeredStore, StoreType};
use log::debug;
use reqwest::blocking::Client;
use tezos_proto::context::TezosStoreType;

use crate::{rollup::rpc_client::StateResponse, Error, Result};

pub struct RpcContext {
    base_url: String,
    client: Client,
    cache: EphemeralStore<TezosStoreType>,
    state_level: u32,
}

impl RpcContext {
    pub fn new(base_url: String, state_level: u32) -> Self {
        Self {
            client: Client::new(),
            cache: EphemeralStore::new(),
            base_url,
            state_level,
        }
    }

    fn get_state_value(&self, key: String) -> Result<Option<TezosStoreType>> {
        let res = self
            .client
            .get(format!(
                "{}/global/block/{}/durable/wasm_2_0_0/value?key={}",
                self.base_url, self.state_level, key
            ))
            .send()?;

        if res.status() == 200 || res.status() == 500 {
            let content: Option<StateResponse> = res.json()?;
            match content {
                Some(StateResponse::Value(value)) => {
                    let payload = hex::decode(value)?;
                    Ok(Some(TezosStoreType::from_vec(payload)?))
                }
                Some(StateResponse::Errors(errors)) => {
                    let message = errors.first().unwrap().to_string();
                    Err(Error::RollupInternalError { message })
                }
                None => Ok(None),
            }
        } else {
            Err(Error::RollupClientError {
                status: res.status().as_u16(),
            })
        }
    }
}

impl LayeredStore<TezosStoreType> for RpcContext {
    fn log(&self, msg: String) {
        debug!("{}", msg)
    }

    fn has(&self, key: String) -> layered_store::Result<bool> {
        match self.cache.has(key.clone())? {
            true => return Ok(true),
            false => {
                if self.cache.pending_removed(&key) {
                    return Ok(false);
                }
            }
        };
        match self.get_state_value(key) {
            Ok(val) => Ok(val.is_some()),
            Err(err) => Err(err_into(err)),
        }
    }

    fn get(&mut self, key: String) -> layered_store::Result<Option<TezosStoreType>> {
        match self.cache.get(key.clone())? {
            Some(val) => return Ok(Some(val)),
            None => {
                if self.cache.pending_removed(&key) {
                    return Ok(None);
                }
            }
        };
        self.get_state_value(key).map_err(err_into)
    }

    fn set(&mut self, key: String, val: Option<TezosStoreType>) -> layered_store::Result<()> {
        self.cache.set(key, val)
    }

    fn has_pending_changes(&self) -> bool {
        self.cache.has_pending_changes()
    }

    fn commit(&mut self) -> layered_store::Result<()> {
        self.cache.commit()
    }

    fn rollback(&mut self) {
        self.cache.rollback()
    }

    fn clear(&mut self) {
        self.cache.clear()
    }
}
