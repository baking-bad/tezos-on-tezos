use tezos_core::types::{
    encoded::{Address, PublicKey, ImplicitAddress, Signature, Encoded}
};
use tezos_michelson::michelson::{
    types::{Type, ComparableType},
    types,
    data::{Data, Instruction},
    data,
};
use tezos_operation::operations::OperationContent;

use crate::{
    Result, Error,
    vm::types::{AddressItem, KeyItem, KeyHashItem, SignatureItem, OperationItem, LambdaItem, StackItem},
    err_type,
    type_check_fn_comparable,
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

    pub fn from_data(data: Data, ty: &Type, parameter_type: &Type, return_type: &Type) -> Result<StackItem> {
        match data {
            Data::Instruction(body) => {
                let lambda = Self::new(parameter_type.clone(), return_type.clone(), body);
                Ok(lambda.into())
            },
            _ => err_type!(ty, data)
        }
    }
        
    pub fn into_data(self, ty: &Type) -> Result<Data> {
        if let Type::Lambda(_) = ty {
            return Ok(Data::Instruction(self.outer_value))
        }
        err_type!(ty, self)
    } 

    pub fn get_type(&self) -> Result<Type> {
        Ok(types::lambda(self.inner_type.0.clone(), self.inner_type.1.clone()))
    }

    pub fn unwrap(self) -> (Instruction, (Type, Type)) {
        (self.outer_value, self.inner_type)
    }
}

impl PartialOrd for AddressItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (&self.0, &other.0) {
            (Address::Implicit(_), Address::Originated(_)) => Some(std::cmp::Ordering::Less),
            (Address::Originated(_), Address::Implicit(_)) => Some(std::cmp::Ordering::Greater),
            (l, r) => l.value().partial_cmp(r.value()),
        }
    }
}

fn public_key_meta(key: &PublicKey) -> (i8, usize) {
    match key {
        PublicKey::Ed25519(_) => (0, 0),
        PublicKey::Secp256K1(_) => (1, 0),
        PublicKey::P256(_) => (2, 1),
    }
}

impl PartialOrd for KeyItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (l, r) = (public_key_meta(&self.0), public_key_meta(&other.0));
        match l.0.partial_cmp(&r.0) {
            Some(std::cmp::Ordering::Equal) => {
                match (self.0.to_bytes().ok(), other.0.to_bytes().ok()) {
                    (Some(self_data), Some(other_data)) => self_data[l.1..].partial_cmp(&other_data[l.1..]),
                    _ => None
                }
            },
            x => x
        }
    }
}

impl PartialOrd for SignatureItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.value().partial_cmp(other.0.value())
    }
}

impl PartialOrd for KeyHashItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.value().partial_cmp(other.0.value())
    }
}