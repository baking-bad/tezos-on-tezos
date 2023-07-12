// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use layered_store::{error::err_into, internal_error, Result, StoreType};
use std::io::Write;

use crate::storage::{Ciphertext, SaplingHead};

impl StoreType for SaplingHead {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 25 {
            return Err(internal_error!(
                "Unexpected SaplingHead byte len: {}",
                bytes.len()
            ));
        }
        Ok(SaplingHead {
            roots_pos: u64::from_be_bytes(bytes[..8].try_into().unwrap()),
            nullifiers_size: u64::from_be_bytes(bytes[8..16].try_into().unwrap()),
            commitments_size: u64::from_be_bytes(bytes[16..24].try_into().unwrap()),
            memo_size: bytes[24],
        })
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(32);
        buf.write(self.roots_pos.to_be_bytes().as_slice())?;
        buf.write(self.nullifiers_size.to_be_bytes().as_slice())?;
        buf.write(self.commitments_size.to_be_bytes().as_slice())?;
        buf.write(&[self.memo_size])?;
        Ok(buf)
    }
}

impl StoreType for Ciphertext {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ciphertext::try_from(bytes).map_err(err_into)
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        self.try_into().map_err(err_into)
    }
}
