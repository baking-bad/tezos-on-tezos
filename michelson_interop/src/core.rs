// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use tezos_michelson::michelson::{
    data::{self, Data},
    types::{self, Type},
};

use crate::{Error, MichelsonInterop, Result};

pub type Bytes = Vec<u8>;

impl MichelsonInterop for () {
    fn michelson_type(field_name: Option<String>) -> Type {
        let ty = types::Unit::new(None);
        match field_name {
            Some(name) => ty.with_field_annotation(name),
            None => ty.into(),
        }
    }

    fn to_michelson(&self) -> Result<Data> {
        Ok(data::unit())
    }

    fn from_michelson(data: Data) -> Result<Self> {
        match data {
            Data::Unit(_) => Ok(()),
            _ => Err(Error::TypeMismatch {
                message: format!("Expected unit, got {:?}", data),
            }),
        }
    }
}

impl MichelsonInterop for bool {
    fn michelson_type(field_name: Option<String>) -> Type {
        let ty = types::Bool::new(None);
        match field_name {
            Some(name) => ty.with_field_annotation(name),
            None => ty.into(),
        }
    }

    fn to_michelson(&self) -> Result<Data> {
        match self {
            true => Ok(data::r#true()),
            false => Ok(data::r#false()),
        }
    }

    fn from_michelson(data: Data) -> Result<Self> {
        match data {
            Data::True(_) => Ok(true),
            Data::False(_) => Ok(false),
            _ => Err(Error::TypeMismatch {
                message: format!("Expected bool, got {:?}", data),
            }),
        }
    }
}

impl MichelsonInterop for String {
    fn michelson_type(field_name: Option<String>) -> Type {
        let ty = types::String::new(None);
        match field_name {
            Some(name) => ty.with_field_annotation(name),
            None => ty.into(),
        }
    }

    fn to_michelson(&self) -> Result<Data> {
        Ok(Data::try_from(self.clone())?)
    }

    fn from_michelson(data: Data) -> Result<Self> {
        match data {
            Data::String(value) => Ok(value.clone().into_string()),
            _ => Err(Error::TypeMismatch {
                message: format!("Expected string, got {:?}", data),
            }),
        }
    }
}

impl MichelsonInterop for Bytes {
    fn michelson_type(field_name: Option<String>) -> Type {
        let ty = types::Bytes::new(None);
        match field_name {
            Some(name) => ty.with_field_annotation(name),
            None => ty.into(),
        }
    }

    fn to_michelson(&self) -> Result<data::Data> {
        let bytes: data::Bytes = data::bytes(self.as_slice());
        Ok(bytes.into())
    }

    fn from_michelson(data: Data) -> Result<Self> {
        match data {
            data::Data::Bytes(bytes) => Ok((&bytes).into()),
            _ => Err(Error::TypeMismatch {
                message: format!("Expected bytes, got {:?}", data),
            }),
        }
    }
}
