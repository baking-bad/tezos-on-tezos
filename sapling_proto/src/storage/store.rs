// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use layered_store::{error::err_into, Result, StoreType};

use crate::storage::{Ciphertext, SaplingHead};

impl StoreType for SaplingHead {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        serde_json_wasm::de::from_slice(bytes).map_err(err_into)
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        serde_json_wasm::ser::to_vec(self).map_err(err_into)
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
