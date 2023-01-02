
use std::fmt::Display;
use tezos_michelson::michelson::{
    data::Data,
    data,
    types::Type,
    types
};

use crate::{
    Result,
    vm::types::{OptionItem, StackItem},
    vm::typechecker::{check_types_equal},
    err_type,
};

impl OptionItem {
    pub fn new(value: Option<Box<StackItem>>, val_type: &Type) -> Self {
        Self { outer_value: value, inner_type: val_type.clone() }
    }

    pub fn none(val_type: &Type) -> Self {
        Self { outer_value: None, inner_type: val_type.clone() }
    }

    pub fn some(val: StackItem) -> Result<Self> {
        Ok(Self { inner_type: val.get_type()?, outer_value: Some(Box::new(val)) })
    }

    pub fn from_data(data: Data, ty: &Type, val_type: &Type) -> Result<StackItem> {
        match data {
            Data::None(_) => Ok(Self::none(val_type).into()),
            Data::Some(val) => {
                let inner = StackItem::from_data(*val.value, val_type)?;
                let outer = Self::new(Some(Box::new(inner)), val_type);
                Ok(outer.into())
            },
            _ => err_type!(ty, data)
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        if let Type::Option(option_ty) = ty {
            return match self.outer_value {
                None => {
                    check_types_equal(&option_ty.r#type, &self.inner_type)?;
                    Ok(Data::None(data::none()))
                },
                Some(val) => {
                    let inner = (*val).into_data(&option_ty.r#type)?;
                    Ok(Data::Some(data::some(inner)))
                }
            }
        }
        err_type!(ty, self)
    }

    pub fn is_none(&self) -> bool {
        self.outer_value.is_none()
    }

    pub fn unwrap(self) -> Option<StackItem> {
        match self.outer_value {
            Some(value) => Some(*value),
            None => None
        }
    }

    pub fn get_type(&self) -> Type {
        types::option(self.inner_type.clone())
    }
}

impl Display for OptionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.outer_value {
            Some(val) => f.write_fmt(format_args!("{}?", &val)),
            None => f.write_str("None")
        }
    }
}

impl Eq for OptionItem {}

impl PartialEq for OptionItem {
    fn eq(&self, rhs: &Self) -> bool {
        self.outer_value == rhs.outer_value
    }
}

impl Ord for OptionItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (&self.outer_value, &other.outer_value) {
            (None, Some(_)) => std::cmp::Ordering::Less,
            (Some(_), None) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
            (Some(lval), Some(rval)) => lval.cmp(&rval)
        }
    }
}

impl PartialOrd for OptionItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}