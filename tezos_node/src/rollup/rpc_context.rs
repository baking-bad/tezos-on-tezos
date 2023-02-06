use log::debug;
use reqwest::blocking::Client;
use tezos_ctx::{ContextNode, EphemeralContext, GenericContext};

use crate::{rollup::rpc_client::StateResponse, Error, Result};

fn err_into(e: impl std::fmt::Debug) -> tezos_ctx::Error {
    tezos_ctx::Error::Internal(tezos_ctx::error::InternalError::new(
        tezos_ctx::error::InternalKind::Store,
        format!("RPC context error: {:?}", e),
    ))
}

pub struct RpcContext {
    base_url: String,
    client: Client,
    tmp_ctx: EphemeralContext,
    state_level: u32,
}

impl RpcContext {
    pub fn new(base_url: String, state_level: u32) -> Self {
        Self {
            client: Client::new(),
            tmp_ctx: EphemeralContext::new(),
            base_url,
            state_level,
        }
    }

    fn get_state_value(&self, key: String) -> Result<Option<ContextNode>> {
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
                    Ok(Some(ContextNode::from_vec(payload)?))
                }
                Some(StateResponse::Errors(errors)) => {
                    let message = errors.first().unwrap().to_string();
                    Err(Error::DurableStorageError { message })
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

impl GenericContext for RpcContext {
    fn log(&self, msg: String) {
        debug!("{}", msg)
    }

    fn has(&self, key: String) -> tezos_ctx::Result<bool> {
        match self.tmp_ctx.has(key.clone())? {
            true => return Ok(true),
            false => {
                if self.tmp_ctx.pending_removed(&key) {
                    return Ok(false);
                }
            }
        };
        match self.get_state_value(key) {
            Ok(val) => Ok(val.is_some()),
            Err(err) => Err(err_into(err)),
        }
    }

    fn get(&mut self, key: String) -> tezos_ctx::Result<Option<ContextNode>> {
        match self.tmp_ctx.get(key.clone())? {
            Some(val) => return Ok(Some(val)),
            None => {
                if self.tmp_ctx.pending_removed(&key) {
                    return Ok(None);
                }
            }
        };
        self.get_state_value(key).map_err(err_into)
    }

    fn set(&mut self, key: String, val: Option<ContextNode>) -> tezos_ctx::Result<()> {
        self.tmp_ctx.set(key, val)
    }

    fn has_pending_changes(&self) -> bool {
        self.tmp_ctx.has_pending_changes()
    }

    fn commit(&mut self) -> tezos_ctx::Result<()> {
        self.tmp_ctx.commit()
    }

    fn rollback(&mut self) {
        self.tmp_ctx.rollback()
    }

    fn clear(&mut self) {
        self.tmp_ctx.clear()
    }
}
