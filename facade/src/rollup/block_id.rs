use tezos_core::types::encoded::{BlockHash, Encoded};

use crate::{Result, Error};

#[derive(Debug, Clone, PartialEq)]
pub enum BlockId {
    Head,
    Genesis,
    Level(i32),
    Hash(BlockHash),
}

impl TryFrom<&str> for BlockId {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "head" => Ok(Self::Head),
            "genesis" => Ok(Self::Genesis),
            hash if hash.len() == 51 => match BlockHash::new(hash.into()) {
                Ok(value) => Ok(Self::Hash(value)),
                Err(err) => Err(Error::InvalidArguments {
                    message: err.to_string(),
                }),
            },
            level => match i32::from_str_radix(level, 10) {
                Ok(value) => Ok(Self::Level(value)),
                Err(err) => Err(Error::InvalidArguments {
                    message: err.to_string(),
                }),
            },
        }
    }
}