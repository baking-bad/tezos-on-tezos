use std::fmt::Display;
use tezos_michelson::michelson::{
    data::Data,
    data,
    types::Type,
};

use crate::{
    Result,
    vm::types::{BigMapItem, StackItem, MapItem, BigMapPtr},
    vm::typechecker::check_types_equal,
    err_type,
};

impl BigMapItem {
    pub fn from_data(data: Data, ty: &Type, key_type: &Type, val_type: &Type) -> Result<StackItem> {
        match data {
            Data::Int(ptr) => {
                let ptr = BigMapPtr { value: ptr.to_integer()?, outer_type: ty.clone() };
                Ok(StackItem::BigMap(Self::Ptr(ptr)))
            },
            Data::Sequence(sequence) => {
                let map = MapItem::from_sequence(sequence, key_type.clone(), val_type.clone())?;
                Ok(StackItem::BigMap(Self::Map(map)))
            },
            _ => err_type!(ty, data)
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        if let Type::BigMap(_) = ty {
            return match self {
                Self::Ptr(ptr) => {
                    check_types_equal(ty, &ptr.outer_type)?;
                    Ok(Data::Int(data::int(ptr.value)))
                },
                Self::Map(_) => err_type!(ty, self)  // NOTE: not supported
            }
        }
        err_type!(ty, self)
    }

    pub fn get_type(&self) -> Result<Type> {
        match self {
            Self::Ptr(ptr) => Ok(ptr.outer_type.clone()),
            Self::Map(map) => map.get_type()
        }
    }
}

impl Display for BigMapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Map(map) => map.fmt(f),
            Self::Ptr(ptr) => f.write_fmt(format_args!("<{}>", ptr.value))
        }
    }
}