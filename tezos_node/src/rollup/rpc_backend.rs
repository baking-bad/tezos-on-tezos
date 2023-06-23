use layered_store::{error::err_into, StoreBackend};
use log::debug;
use reqwest::blocking::Client;

use crate::{rollup::rpc_client::StateResponse, Error, Result};

pub struct RpcBackend {
    base_url: String,
    client: Client,
    state_level: u32,
}

impl RpcBackend {
    pub fn new(base_url: String, state_level: u32) -> Self {
        Self {
            client: Client::new(),
            base_url,
            state_level,
        }
    }

    fn store_get(&self, key: &str) -> Result<Option<Vec<u8>>> {
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
                    Ok(Some(payload))
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

impl StoreBackend for RpcBackend {
    fn default() -> Self {
        unimplemented!()
    }

    fn log(&self, msg: &str) {
        debug!("{}", msg)
    }

    fn has(&self, key: &str) -> layered_store::Result<bool> {
        // TODO: more optimal check without fetching data
        match self.store_get(key) {
            Ok(val) => Ok(val.is_some()),
            Err(err) => Err(err_into(err)),
        }
    }

    fn read(&self, key: &str) -> layered_store::Result<Option<Vec<u8>>> {
        self.store_get(key).map_err(err_into)
    }

    fn write(&mut self, _key: &str, _val: &[u8]) -> layered_store::Result<()> {
        Ok(())
    }

    fn delete(&mut self, _key: &str) -> layered_store::Result<()> {
        Ok(())
    }

    fn clear(&mut self) {}
}
