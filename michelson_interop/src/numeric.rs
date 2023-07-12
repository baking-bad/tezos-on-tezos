// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use crate::{Error, MichelsonInterop, Result};
use ibig::{IBig, UBig};
use tezos_core::types::{
    mutez::Mutez,
    number::{Int, Nat},
};
use tezos_michelson::michelson::{
    data::Data,
    types::{self, Type},
};

impl MichelsonInterop for Mutez {
    fn michelson_type(field_name: Option<String>) -> Type {
        let ty = types::Mutez::new(None);
        match field_name {
            Some(name) => ty.with_field_annotation(name),
            None => ty.into(),
        }
    }

    fn to_michelson(&self) -> Result<Data> {
        Ok(Data::Int(self.try_into()?))
    }

    fn from_michelson(data: Data) -> Result<Self> {
        match data {
            Data::Int(value) => Ok((&value).try_into()?),
            _ => Err(Error::TypeMismatch {
                message: format!("Expected int (mutez), got {:?}", data),
            }),
        }
    }
}

impl MichelsonInterop for u64 {
    fn michelson_type(field_name: Option<String>) -> Type {
        let ty = types::Mutez::new(None);
        match field_name {
            Some(name) => ty.with_field_annotation(name),
            None => ty.into(),
        }
    }

    fn to_michelson(&self) -> Result<Data> {
        let int = *self as i64;
        if int < 0 {
            return Err(Error::MutezOverflow);
        }
        Ok(Data::Int(int.into()))
    }

    fn from_michelson(data: Data) -> Result<Self> {
        match data {
            Data::Int(value) => {
                let int: i64 = value.to_integer()?;
                Ok(int as u64)
            }
            _ => Err(Error::TypeMismatch {
                message: format!("Expected int (mutez), got {:?}", data),
            }),
        }
    }
}

impl MichelsonInterop for Int {
    fn michelson_type(field_name: Option<String>) -> Type {
        let ty = types::Int::new(None);
        match field_name {
            Some(name) => ty.with_field_annotation(name),
            None => ty.into(),
        }
    }

    fn to_michelson(&self) -> Result<Data> {
        Ok(Data::Int(self.clone()))
    }

    fn from_michelson(data: Data) -> Result<Self> {
        match data {
            Data::Int(value) => Ok(value.clone()),
            _ => Err(Error::TypeMismatch {
                message: format!("Expected int, got {:?}", data),
            }),
        }
    }
}

impl MichelsonInterop for IBig {
    fn michelson_type(field_name: Option<String>) -> Type {
        let ty = types::Int::new(None);
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
            Data::Int(value) => Ok(value.into()),
            _ => Err(Error::TypeMismatch {
                message: format!("Expected int, got {:?}", data),
            }),
        }
    }
}

impl MichelsonInterop for Nat {
    fn michelson_type(field_name: Option<String>) -> Type {
        let ty = types::Nat::new(None);
        match field_name {
            Some(name) => ty.with_field_annotation(name),
            None => ty.into(),
        }
    }

    fn to_michelson(&self) -> Result<Data> {
        Ok(Data::Int(self.into()))
    }

    fn from_michelson(data: Data) -> Result<Self> {
        match data {
            Data::Int(value) => Ok(value.try_into()?),
            Data::Nat(value) => Ok(value.clone()),
            _ => Err(Error::TypeMismatch {
                message: format!("Expected int, got {:?}", data),
            }),
        }
    }
}

impl MichelsonInterop for UBig {
    fn michelson_type(field_name: Option<String>) -> Type {
        let ty = types::Nat::new(None);
        match field_name {
            Some(name) => ty.with_field_annotation(name),
            None => ty.into(),
        }
    }

    fn to_michelson(&self) -> Result<Data> {
        let int: IBig = self.into();
        Ok(Data::Int(int.into()))
    }

    fn from_michelson(data: Data) -> Result<Self> {
        match data {
            Data::Int(value) => Ok(value.try_into()?),
            Data::Nat(value) => Ok(value.into()),
            _ => Err(Error::TypeMismatch {
                message: format!("Expected int, got {:?}", data),
            }),
        }
    }
}
