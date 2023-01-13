use std::fmt::Display;
use tezos_michelson::michelson::{
    data::Data,
    data,
    types::Type,
    types
};

use crate::{
    Result,
    types::{OrItem, StackItem, OrVariant},
    typechecker::{check_types_equal},
    err_type,
    type_cast
};

impl OrItem {
    pub fn left(left_val: StackItem, right_type: Type) -> Self {
        let var = OrVariant { value: Box::new(left_val), other_type: right_type };
        Self::Left(var)
    }

    pub fn right(right_val: StackItem, left_type: Type) -> Self {
        let var = OrVariant { value: Box::new(right_val), other_type: left_type };
        Self::Right(var)
    }

    pub fn from_data(data: Data, left_type: &Type, right_type: &Type) -> Result<StackItem> {
        match data {
            Data::Left(left) => {
                let inner = StackItem::from_data(*left.value, left_type)?;
                Ok(StackItem::Or(Self::left(inner, right_type.clone())))
            },
            Data::Right(right) => {
                let inner = StackItem::from_data(*right.value, right_type)?;
                Ok(StackItem::Or(Self::right(inner, left_type.clone())))
            },
            _ => err_type!("Data::Left or Data::Right", data)
        }
    }
    
    pub fn into_data(self, ty: &Type) -> Result<Data> {
        let ty = type_cast!(ty, Or)?;
        match self {
            Self::Left(var) => {
                check_types_equal(&ty.rhs, &var.other_type)?;
                let inner = var.value.into_data(&ty.lhs)?;
                Ok(Data::Left(data::left(inner)))
            },
            Self::Right(var) => {
                check_types_equal(&ty.lhs, &var.other_type)?;
                let inner = var.value.into_data(&ty.rhs)?;
                Ok(Data::Right(data::right(inner)))
            }
        }
    }

    pub fn is_left(&self) -> bool {
        match self {
            Self::Left(_) => true,
            Self::Right(_) => false
        }
    }

    pub fn unwrap(self) -> StackItem {
        match self {
            Self::Left(var) => *var.value,
            Self::Right(var) => *var.value
        }
    }

    pub fn get_type(&self) -> Result<Type> {
        let (lhs, rhs) = match self {
            Self::Left(var) => (var.value.get_type()?, var.other_type.clone()),
            Self::Right(var) => (var.other_type.clone(), var.value.get_type()?)
        };
        Ok(types::or(lhs, rhs))
    }
}

impl Display for OrItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left(var) => f.write_fmt(format_args!("({} + _)", var.value)),
            Self::Right(var) => f.write_fmt(format_args!("(_ + {})", var.value))
        }
    }
}

impl Eq for OrVariant {}

impl PartialEq for OrVariant {
    fn eq(&self, rhs: &Self) -> bool {
        self.value == rhs.value
    }
}

impl Ord for OrVariant {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for OrVariant {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}