use std::fmt::Display;
use tezos_michelson::michelson::{
    data::Data,
    data,
    types::Type,
    types
};

use crate::{
    Result,
    types::{BigMapItem, StackItem, MapItem, BigMapPtr},
    typechecker::check_types_equal,
    err_type,
};

impl BigMapPtr {
    pub fn new(ptr: i64, key_type: Type, val_type: Type) -> Self {
        Self { ptr, inner_type: (key_type, val_type), diff: Vec::new() }
    }
}

impl BigMapItem {
    pub fn from_data(data: Data, key_type: &Type, val_type: &Type) -> Result<StackItem> {
        match data {
            Data::Int(ptr) => {
                let ptr = BigMapPtr::new(ptr.to_integer()?, key_type.clone(), val_type.clone());
                Ok(StackItem::BigMap(Self::Ptr(ptr)))
            },
            Data::Sequence(sequence) => {
                let map = MapItem::from_sequence(sequence, key_type.clone(), val_type.clone())?;
                Ok(StackItem::BigMap(Self::Map(map)))
            },
            _ => err_type!("Data::Int or Data::Sequence", data)
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        if let Type::BigMap(big_map_ty) = ty {
            return match self {
                Self::Ptr(ptr) => {
                    check_types_equal(&big_map_ty.key_type, &ptr.inner_type.0)?;
                    check_types_equal(&big_map_ty.value_type, &ptr.inner_type.1)?;
                    Ok(Data::Int(data::int(ptr.ptr)))
                },
                Self::Map(_) => err_type!(ty, self)  // NOTE: not supported
            }
        }
        err_type!(ty, self)
    }

    pub fn get_type(&self) -> Result<Type> {
        match self {
            Self::Ptr(ptr) => {
                Ok(types::big_map(ptr.inner_type.0.clone(), ptr.inner_type.1.clone()))
            },
            Self::Map(map) => map.get_type()
        }
    }
}

impl Display for BigMapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Map(map) => map.fmt(f),
            Self::Ptr(ptr) => f.write_fmt(format_args!("${}", ptr.ptr))
        }
    }
}