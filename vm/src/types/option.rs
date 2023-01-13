
use std::fmt::Display;
use tezos_michelson::michelson::{
    data::Data,
    data,
    types::Type,
    types
};

use crate::{
    Result,
    types::{OptionItem, StackItem},
    typechecker::{check_types_equal},
    err_type,
    type_cast
};

impl OptionItem {
    pub fn some(item: StackItem) -> Self {
        Self::Some(Box::new(item))
    }
    
    pub fn none(ty: &Type) -> Self {
        Self::None(ty.clone())
    }

    pub fn from_data(data: Data, val_type: &Type) -> Result<StackItem> {
        match data {
            Data::None(_) => Ok(Self::None(val_type.clone()).into()),
            Data::Some(val) => {
                let inner = StackItem::from_data(*val.value, val_type)?;
                Ok(Self::Some(Box::new(inner)).into())
            },
            _ => err_type!("Data::None or Data::Some", data)
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        let ty = type_cast!(ty, Option)?;
        match self {
            Self::None(inner_ty) => {
                check_types_equal(&ty.r#type, &inner_ty)?;
                Ok(Data::None(data::none()))
            },
            Self::Some(val) => {
                let inner = (*val).into_data(&ty.r#type)?;
                Ok(Data::Some(data::some(inner)))
            }
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            Self::None(_) => true,
            Self::Some(_) => false
        }
    }

    pub fn unwrap(self) -> Option<StackItem> {
        match self {
            Self::Some(value) => Some(*value),
            Self::None(_) => None
        }
    }

    pub fn get_type(&self) -> Result<Type> {
        match self {
            Self::None(ty) => Ok(ty.clone()),
            Self::Some(inner) => Ok(types::option(inner.get_type()?))
        }        
    }
}

impl Display for OptionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Some(val) => f.write_fmt(format_args!("{}?", &val)),
            Self::None(_) => f.write_str("None")
        }
    }
}

impl Eq for OptionItem {}

impl PartialEq for OptionItem {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Self::None(_), Self::None(_)) => true,
            (Self::Some(lval), Self::Some(rval)) => lval == rval,
            _ => false
        }
    }
}

impl Ord for OptionItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::None(_), Self::Some(_)) => std::cmp::Ordering::Less,
            (Self::Some(_), Self::None(_)) => std::cmp::Ordering::Greater,
            (Self::None(_), Self::None(_)) => std::cmp::Ordering::Equal,
            (Self::Some(lval), Self::Some(rval)) => lval.cmp(&rval)
        }
    }
}

impl PartialOrd for OptionItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}