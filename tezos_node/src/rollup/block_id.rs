// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use tezos_core::types::encoded::{BlockHash, Encoded};

use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum BlockId {
    Head,
    Level(u32),
    Offset(u32),
    Hash(BlockHash),
}

fn is_offset(value: &str) -> bool {
    value.starts_with("head~") || value.starts_with("head-")
}

impl TryFrom<&str> for BlockId {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "head" => Ok(Self::Head),
            offset if is_offset(offset) => {
                let val = offset
                    .trim_start_matches("head~")
                    .trim_start_matches("head-");
                match u32::from_str_radix(val, 10) {
                    Ok(value) => Ok(Self::Offset(value)),
                    Err(err) => Err(Error::InvalidArguments {
                        message: err.to_string(),
                    }),
                }
            }
            hash if hash.len() == 51 => match BlockHash::new(hash.into()) {
                Ok(value) => Ok(Self::Hash(value)),
                Err(err) => Err(Error::InvalidArguments {
                    message: err.to_string(),
                }),
            },
            level => match u32::from_str_radix(level, 10) {
                Ok(value) => Ok(Self::Level(value)),
                Err(err) => Err(Error::InvalidArguments {
                    message: err.to_string(),
                }),
            },
        }
    }
}
