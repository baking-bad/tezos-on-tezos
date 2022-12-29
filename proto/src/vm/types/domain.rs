use tezos_core::types::{
    encoded::{Address, PublicKey, ImplicitAddress, Signature, Encoded}
};
use tezos_michelson::michelson::{
    types::{Type, ComparableType},
    data::{Data, Instruction},
    data,
};
use tezos_operation::operations::OperationContent;

use crate::{
    Result, Error,
    vm::types::{AddressItem, KeyItem, KeyHashItem, SignatureItem, OperationItem, LambdaItem, StackItem},
    err_type,
    type_check_fn_comparable
};

macro_rules! impl_for_encoded {
    ($item_ty: ident, $impl_ty: ty, $cmp_ty: ident) => {
        impl $item_ty {
            type_check_fn_comparable!($cmp_ty);

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
    pub fn into_content(self) -> OperationContent {
        self.0
    }
}

impl LambdaItem {
    pub fn new(param_type: Type, return_type: Type, body: Instruction) -> Self {
        Self { outer_value: body, inner_type: (param_type, return_type) }
    }

    pub fn unwrap(self) -> (Instruction, (Type, Type)) {
        (self.outer_value, self.inner_type)
    }
}