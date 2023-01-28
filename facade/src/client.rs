use reqwest::Client;
use serde::Deserialize;
use std::str::FromStr;

use crate::{
    Result,
    Error,
};

#[derive(Debug, Clone, PartialEq)]
pub enum BlockId {
    Head,
    Genesis,
    Level(i32),
}

impl TryFrom<&str> for BlockId {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "head" => Ok(Self::Head),
            "genesis" => Ok(Self::Genesis),
            level => Ok(Self::Level(i32::from_str_radix(level, 10)?))
        } 
    }
}

impl FromStr for BlockId {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        Self::try_from(value)
    }
}

impl std::fmt::Display for BlockId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Genesis => f.write_str("genesis"),
            Self::Head => f.write_str("head"),
            Self::Level(level) => f.write_str(&level.to_string())
        }
    }
}

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

pub struct RollupClient {
    base_url: String,
    client: Client
}

impl RollupClient {
    pub fn default() -> Self {
        Self {
            base_url: "http://localhost:8932".into(),
            client: Client::new()
        }
    }

    pub async fn get_state_value(&self, key: String, block_id: BlockId) -> Result<Vec<u8>> {
        let res = self.client
            .get(format!("{}/global/block/{}/durable/wasm_2_0_0/value?key={}", self.base_url, block_id, key))
            .send()
            .await?;

        if res.status() == 200 || res.status() == 500 {
            let content: Option<StateResponse> = res.json().await?;
            match content {
                Some(StateResponse::Value(value)) => {
                    let payload = hex::decode(value)?;
                    Ok(payload)
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