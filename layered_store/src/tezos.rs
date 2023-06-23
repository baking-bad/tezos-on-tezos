use tezos_core::types::{
    encoded::{ContractAddress, Encoded, OperationHash, PublicKey},
    mutez::Mutez,
    number::Nat,
};
use tezos_michelson::micheline::Micheline;

use crate::{internal_error, Result, StoreType};

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
