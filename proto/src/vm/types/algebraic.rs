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

impl OrItem {
    pub fn left(left_val: StackItem, right_type: &Type) -> Self {
        let var = OrVariant { value: Box::new(left_val), other_type: right_type.clone() };
        Self::Left(var)
    }

    pub fn right(right_val: StackItem, left_type: &Type) -> Self {
        let var = OrVariant { value: Box::new(right_val), other_type: left_type.clone() };
        Self::Right(var)
    }

    pub fn from_data(data: Data, ty: &Type, left_type: &Type, right_type: &Type) -> Result<StackItem> {
        match data {
            Data::Left(left) => {
                let inner = StackItem::from_data(*left.value, left_type)?;
                Ok(StackItem::Or(Self::left(inner, right_type)))
            },
            Data::Right(right) => {
                let inner = StackItem::from_data(*right.value, right_type)?;
                Ok(StackItem::Or(Self::right(inner, left_type)))
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

    pub fn from_items(mut items: Vec<StackItem>) -> Self {
        match items.len() {
            2 => Self::new(items.remove(0), items.remove(0)),
            n if n > 2 => Self::new(items.remove(0), Self::from_items(items).into()),
            _ => unreachable!("invalid number of args")
        }
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

    pub fn into_items(self, arity: usize) -> Result<Vec<StackItem>> {
        match arity {
            2 => Ok(vec![self.0.0, self.0.1]),
            n if n > 2 => {
                let mut items = match self.0.1 {
                    StackItem::Pair(inner_pair) => inner_pair.into_items(arity - 1)?,
                    item => return err_type!("Inner pair", item)
                };
                items.insert(0, self.0.0);
                Ok(items)
            },
            _ => Err(Error::UnexpectedPairArity)
        }
    }

    pub fn unpair(self) -> (StackItem, StackItem) {
        (self.0.0, self.0.1)
    }

    pub fn get(&self, idx: usize) -> Result<StackItem> {
        match idx {
            0 => Ok(self.0.0.clone()),
            1 => Ok(self.0.1.clone()),
            _ => match &self.0.1 {
                StackItem::Pair(inner_pair) => inner_pair.get(idx - 1),
                item => err_type!("Inner pair", item)
            }
        }
    }

    pub fn update(self, idx: usize, item: StackItem) -> Result<Self> {
        match idx {
            0 => Ok(Self::new(item, self.0.1)),
            1 => Ok(Self::new(self.0.0, item)),
            _ => match self.0.1 {
                StackItem::Pair(inner_pair) => inner_pair.update(idx - 1, item),
                item => err_type!("Inner pair", item)
            }
        }
    }

    pub fn get_type(&self) -> Result<Type> {
        Ok(types::pair(vec![self.0.0.get_type()?, self.0.1.get_type()?]))
    }
}

// NOTE: assuming that type checks have been made prior to comparison (comparable, equality)

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