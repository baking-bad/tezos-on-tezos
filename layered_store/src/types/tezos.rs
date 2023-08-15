// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use tezos_core::types::{
    encoded::{ContractAddress, Encoded, OperationHash, PublicKey},
    mutez::Mutez,
    number::Nat,
};
use tezos_michelson::micheline::Micheline;
use tezos_operation::operations::SignedOperation;
use tezos_rpc::models::operation::Operation;

use crate::{error::err_into, internal_error, Result, StoreType};

macro_rules! impl_for_core {
    ($cls: ident, $ty: ty) => {
        impl StoreType for $ty {
            fn from_bytes(bytes: &[u8]) -> Result<Self> {
                $cls::from_bytes(bytes).map_err(|e| internal_error!("{:?}", e))
            }

            fn to_bytes(&self) -> Result<Vec<u8>> {
                $cls::to_bytes(self).map_err(|e| internal_error!("{:?}", e))
            }
        }
    };
}

impl_for_core!(Encoded, PublicKey);
impl_for_core!(Encoded, OperationHash);
impl_for_core!(Encoded, ContractAddress);
impl_for_core!(Micheline, Micheline);
impl_for_core!(Mutez, Mutez);
impl_for_core!(Nat, Nat);

impl StoreType for Operation {
    fn from_bytes(_bytes: &[u8]) -> Result<Self> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(serde_json_wasm::de::from_slice(_bytes).map_err(err_into)?)
        }
        #[cfg(target_arch = "wasm32")]
        unimplemented!()
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        serde_json_wasm::ser::to_vec(&self).map_err(err_into)
    }
}

impl StoreType for SignedOperation {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        SignedOperation::from_bytes(bytes).map_err(err_into)
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        SignedOperation::to_bytes(&self).map_err(err_into)
    }
}