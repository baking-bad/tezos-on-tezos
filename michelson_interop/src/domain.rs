// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

#![allow(type_alias_bounds)]

use chrono::DateTime;
use std::hash::Hash;
use tezos_core::types::{
    encoded::{Address, ChainId, Encoded, ImplicitAddress, Key, Signature},
    number::Nat,
};
use tezos_michelson::michelson::{
    data::Data,
    types::{self, Type},
};

use crate::{Error, MichelsonInterop, Result};

pub type Ticket<T: MichelsonInterop + Hash + Eq> = (Address, T, Nat);

macro_rules! impl_for_encoded {
    ($ty: ty, $fn: ident) => {
        impl MichelsonInterop for $ty {
            fn michelson_type(field_name: Option<String>) -> Type {
                let ty = types::$fn::new(None);
                match field_name {
                    Some(name) => ty.with_field_annotation(name),
                    None => ty.into(),
                }
            }

            fn to_michelson(&self) -> Result<Data> {
                Ok(Data::String(self.into_string().try_into()?))
            }

            fn from_michelson(data: Data) -> Result<Self> {
                match data {
                    Data::String(value) => Ok(Self::try_from(value.clone().into_string())?),
                    Data::Bytes(value) => {
                        let bytes: Vec<u8> = (&value).into();
                        Ok(Self::from_bytes(bytes.as_slice())?)
                    }
                    _ => Err(Error::TypeMismatch {
                        message: format!("Expected {}, got {:?}", stringify!($fn), data),
                    }),
                }
            }
        }
    };
}

impl_for_encoded!(Address, Address);
impl_for_encoded!(ChainId, ChainId);
impl_for_encoded!(Key, Key);
impl_for_encoded!(ImplicitAddress, KeyHash);
impl_for_encoded!(Signature, Signature);

impl MichelsonInterop for i64 {
    fn michelson_type(field_name: Option<String>) -> Type {
        let ty = types::Timestamp::new(None);
        match field_name {
            Some(name) => ty.with_field_annotation(name),
            None => ty.into(),
        }
    }

    fn to_michelson(&self) -> Result<Data> {
        Ok(Data::Int(self.clone().into()))
    }

    fn from_michelson(data: Data) -> Result<Self> {
        match data {
            Data::Int(value) => Ok(value.to_integer()?),
            Data::String(value) => Ok(DateTime::parse_from_rfc3339(value.to_str())?.timestamp()),
            _ => Err(Error::TypeMismatch {
                message: format!("Expected unix time or rfc3339, got {:?}", data),
            }),
        }
    }
}
