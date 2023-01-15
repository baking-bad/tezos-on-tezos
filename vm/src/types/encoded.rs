use std::fmt::Display;
use tezos_core::types::{
    encoded::{Address, PublicKey, ImplicitAddress, Signature, ChainId, Encoded}
};
use tezos_michelson::michelson::{
    types::{Type, ComparableType},
    data::Data,
    data,
};

use crate::{
    Result,
    types::{AddressItem, KeyItem, KeyHashItem, SignatureItem, ChainIdItem, StackItem},
    formatter::Formatter,
    err_mismatch,
    comparable_type_cast
};

macro_rules! impl_for_encoded {
    ($item_ty: ident, $impl_ty: ty, $cmp_ty: ident) => {
        impl $item_ty {
            pub fn new(value: $impl_ty) -> Self {
                Self(value)
            }

            pub fn from_data(data: Data) -> Result<StackItem> {
                match data {
                    Data::String(val) => Ok($item_ty(<$impl_ty>::new(val.into_string())?).into()),
                    Data::Bytes(val) => {
                        let bytes: Vec<u8> = (&val).into();
                        Ok($item_ty(<$impl_ty>::from_bytes(bytes.as_slice())?).into())   
                    },
                    _ => err_mismatch!("String or Bytes", data.format())
                }
            }
                
            pub fn into_data(self, ty: &Type) -> Result<Data> {
                comparable_type_cast!(ty, $cmp_ty);
                Ok(Data::String(data::String::from_string(self.0.into_string())?))
            }

            pub fn unwrap(self) -> $impl_ty {
                self.0
            }
        }

        impl PartialOrd for $item_ty {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Display for $item_ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.0.value())
            }
        }
    };
}

impl_for_encoded!(AddressItem, Address, Address);
impl_for_encoded!(KeyItem, PublicKey, Key);
impl_for_encoded!(KeyHashItem, ImplicitAddress, KeyHash);
impl_for_encoded!(SignatureItem, Signature, Signature);
impl_for_encoded!(ChainIdItem, ChainId, ChainId);

impl Ord for AddressItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (&self.0, &other.0) {
            (Address::Implicit(_), Address::Originated(_)) => std::cmp::Ordering::Less,
            (Address::Originated(_), Address::Implicit(_)) => std::cmp::Ordering::Greater,
            (l, r) => l.value().cmp(r.value()),
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

impl Ord for KeyItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (l, r) = (public_key_meta(&self.0), public_key_meta(&other.0));
        let res = l.0.cmp(&r.0);
        if res == std::cmp::Ordering::Equal {
            match (self.0.to_bytes().ok(), other.0.to_bytes().ok()) {
                (Some(self_data), Some(other_data)) => return self_data[l.1..].cmp(&other_data[r.1..]),
                _ => unreachable!("Invalid keys")
            }
        }
        res
    }
}

impl Ord for SignatureItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.value().cmp(other.0.value())
    }
}

impl Ord for ChainIdItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.value().cmp(other.0.value())
    }
}

impl Ord for KeyHashItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.value().cmp(other.0.value())
    }
}
