use tezos_michelson::michelson::{
    data::Data,
    data,
    types::{Type, ComparableType},
    types
};

use crate::{
    Result, Error,
    vm::types::{OptionItem, OrItem, PairItem, UnitItem, StackItem, OrVariant},
    vm::typechecker::{check_types_equal},
    err_type,
    type_check_fn_comparable
};

impl UnitItem {
    type_check_fn_comparable!(Unit);

    pub fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        match data {
            Data::Unit(_) => Ok(StackItem::Unit(Self(()))),
            _ => err_type!(ty, data)
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        self.type_check(ty)?;
        Ok(Data::Unit(data::unit()))
    }
}

impl OptionItem {
    pub fn from_data(data: Data, ty: &Type, val_type: &Type) -> Result<StackItem> {
        match data {
            Data::None(_) => Ok(StackItem::Option(Self {
                outer_value: None,
                inner_type: val_type.clone()
            })),
            Data::Some(val) => {
                let inner = StackItem::from_data(*val.value, val_type)?;
                Ok(StackItem::Option(Self {
                    outer_value: Some(Box::new(inner)),
                    inner_type: val_type.clone()
                }))
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

impl OrItem {
    pub fn from_data(data: Data, ty: &Type, left_type: &Type, right_type: &Type) -> Result<StackItem> {
        match data {
            Data::Left(left) => {
                let inner = StackItem::from_data(*left.value, left_type)?;
                let var = OrVariant { value: Box::new(inner), other_type: right_type.clone() };
                Ok(StackItem::Or(Self::Left(var)))
            },
            Data::Right(right) => {
                let inner = StackItem::from_data(*right.value, right_type)?;
                let var = OrVariant { value: Box::new(inner), other_type: left_type.clone() };
                Ok(StackItem::Or(Self::Right(var)))
            },
            _ => err_type!(ty, data)
        }
    }
    
    pub fn into_data(self, ty: &Type) -> Result<Data> {
        if let Type::Or(or_ty) = ty {
            return match self {
                Self::Left(var) => {
                    check_types_equal(&or_ty.rhs, &var.other_type)?;
                    let inner = var.value.into_data(&or_ty.lhs)?;
                    Ok(Data::Left(data::left(inner)))
                },
                Self::Right(var) => {
                    check_types_equal(&or_ty.lhs, &var.other_type)?;
                    let inner = var.value.into_data(&or_ty.rhs)?;
                    Ok(Data::Right(data::right(inner)))
                }
            }
        }
        err_type!(ty, self)
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

impl PairItem {
    pub fn new(first: StackItem, second: StackItem) -> Self {
        Self(Box::new((first, second)))
    }

    pub fn from_data(data: Data, ty: &Type, first_type: &Type, second_type: &Type) -> Result<StackItem> {
        match data {
            Data::Pair(mut pair) => {
                assert_eq!(2, pair.values.len());
                let first = StackItem::from_data(pair.values.remove(0), first_type)?;
                let second = StackItem::from_data(pair.values.remove(0), second_type)?;
                Ok(StackItem::Pair(Self::new(first, second)))
            },
            _ => err_type!(ty, data)
        }
    }
    
    pub fn into_data(self, ty: &Type) -> Result<Data> {
        if let Type::Pair(pair_ty) = ty {
            assert_eq!(2, pair_ty.types.len());
            let first = self.0.0.into_data(&pair_ty.types[0])?;
            let second = self.0.1.into_data(&pair_ty.types[1])?;
            return Ok(Data::Pair(data::pair(vec![first, second])))
        }
        err_type!(ty, self)
    }

    pub fn unpair(self) -> (StackItem, StackItem) {
        (self.0.0, self.0.1)
    }

    pub fn get_type(&self) -> Result<Type> {
        Ok(types::pair(vec![self.0.0.get_type()?, self.0.1.get_type()?]))
    }
}

// NOTE: assuming that type checks have been made prior to comparison (comparable, equality)

impl PartialEq for OptionItem {
    fn eq(&self, rhs: &Self) -> bool {
        self.outer_value == rhs.outer_value
    }
}

impl PartialEq for OrVariant {
    fn eq(&self, rhs: &Self) -> bool {
        self.value == rhs.value
    }
}

impl PartialOrd for OptionItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (&self.outer_value, &other.outer_value) {
            (None, Some(_)) => Some(std::cmp::Ordering::Less),
            (Some(_), None) => Some(std::cmp::Ordering::Greater),
            (None, None) => Some(std::cmp::Ordering::Equal),
            (Some(lval), Some(rval)) => lval.partial_cmp(&rval)
        }
    }
}

impl PartialOrd for OrVariant {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}