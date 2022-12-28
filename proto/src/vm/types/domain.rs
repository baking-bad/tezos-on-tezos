use tezos_core::types::{
    encoded::{Address, PublicKey, ImplicitAddress, Signature, Encoded}
};
use tezos_michelson::michelson::{
    data::Data, data,
    types::{Type, ComparableType}
};

use crate::{
    Result, Error,
    vm::types::{AddressItem, KeyItem, KeyHashItem, SignatureItem, OperationItem, StackItem},
    err_type,
    type_check_comparable,
};

macro_rules! impl_for_encoded {
    ($item_ty: ident, $impl_ty: ty, $cmp_ty: ident) => {
        impl $item_ty {
            type_check_comparable!($cmp_ty);

            pub fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
                match data {
                    Data::String(val) => Ok($item_ty(<$impl_ty>::new(val.into_string())?).into()),
                    _ => err_type!(ty, data)
                }
            }
                
            pub fn into_data(self, ty: &Type) -> Result<Data> {
                self.type_check(ty)?;
                Ok(Data::String(data::String::from_string(self.0.into_string())?))
            }
        }
    };
}

impl_for_encoded!(AddressItem, Address, Address);
impl_for_encoded!(KeyItem, PublicKey, Key);
impl_for_encoded!(KeyHashItem, ImplicitAddress, KeyHash);
impl_for_encoded!(SignatureItem, Signature, Signature);

impl OperationItem {
    pub fn into_content() {
        
    }
}