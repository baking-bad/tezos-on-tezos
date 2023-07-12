// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use layered_store::{error::err_into, Result, StoreType};
use tezos_rpc::models::operation::Operation;

use crate::{context::batch::BatchReceipt, context::head::Head};

impl StoreType for Head {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        serde_json_wasm::de::from_slice(bytes).map_err(err_into)
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        serde_json_wasm::ser::to_vec(self).map_err(err_into)
    }
}

impl StoreType for BatchReceipt {
    fn from_bytes(_bytes: &[u8]) -> Result<Self> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // This is a workaround to avoid floating point operations introduced by serde.
            // Since we do not need RPC models deserialization inside the kernel,
            // we can only enable that for tests and binaries that are not compiled to wasm.
            serde_json_wasm::de::from_slice(_bytes).map_err(err_into)
        }
        #[cfg(target_arch = "wasm32")]
        unimplemented!()
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        serde_json_wasm::ser::to_vec(self).map_err(err_into)
    }
}

#[derive(Clone, Debug)]
pub struct OperationReceipt(pub Operation);

impl StoreType for OperationReceipt {
    fn from_bytes(_bytes: &[u8]) -> Result<Self> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let operation: Operation = serde_json_wasm::de::from_slice(_bytes).map_err(err_into)?;
            Ok(OperationReceipt(operation))
        }
        #[cfg(target_arch = "wasm32")]
        unimplemented!()
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        serde_json_wasm::ser::to_vec(&self.0).map_err(err_into)
    }
}
