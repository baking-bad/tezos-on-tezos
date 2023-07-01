// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use crate::{error::err_into, internal_error, Result, StoreType};

impl StoreType for i64 {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() == 8 {
            Ok(i64::from_be_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]))
        } else {
            Err(internal_error!("Invalid byte length"))
        }
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(self.to_be_bytes().to_vec())
    }
}

impl StoreType for u64 {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() == 8 {
            Ok(u64::from_be_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]))
        } else {
            Err(internal_error!("Invalid byte length"))
        }
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(self.to_be_bytes().to_vec())
    }
}

impl StoreType for [u8; 32] {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() == 32 {
            Self::try_from(bytes).map_err(err_into)
        } else {
            Err(internal_error!(
                "Invalid byte length: {} (expected 32)",
                bytes.len()
            ))
        }
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(self.to_vec())
    }
}
