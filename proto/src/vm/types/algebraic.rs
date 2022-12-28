use tezos_michelson::michelson::{
    data::Data,
    data,
    types::Type
};

use crate::{
    Result, Error,
    vm::types::{OptionItem, OrItem, PairItem, StackItem},
    err_type, assert_types_equal
};

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
                    assert_types_equal!(*option_ty.r#type, self.inner_type);
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
}

impl OrItem {
    pub fn from_data(data: Data, ty: &Type, left_type: &Type, right_type: &Type) -> Result<StackItem> {
        match data {
            Data::Left(left) => {
                let inner = StackItem::from_data(*left.value, left_type)?;
                Ok(StackItem::Or(Self::Left {
                    value: Box::new(inner),
                    right_type: right_type.clone()
                }))
            },
            Data::Right(right) => {
                let inner = StackItem::from_data(*right.value, right_type)?;
                Ok(StackItem::Or(Self::Right {
                    value: Box::new(inner),
                    left_type: left_type.clone()
                }))
            },
            _ => err_type!(ty, data)
        }
    }
    
    pub fn into_data(self, ty: &Type) -> Result<Data> {
        if let Type::Or(or_ty) = ty {
            return match self {
                Self::Left { value, right_type } => {
                    assert_types_equal!(*or_ty.rhs, right_type);
                    let inner = value.into_data(&or_ty.lhs)?;
                    Ok(Data::Left(data::left(inner)))
                },
                Self::Right { value, left_type } => {
                    assert_types_equal!(*or_ty.lhs, left_type);
                    let inner = value.into_data(&or_ty.rhs)?;
                    Ok(Data::Right(data::right(inner)))
                }
            }
        }
        err_type!(ty, self)
    }
}

impl PairItem {
    pub fn new(first: StackItem, second: StackItem) -> Self {
        Self(Box::new((first, second)))
    }

    pub fn unpair(self) -> (StackItem, StackItem) {
        (self.0.0, self.0.1)
    } 

    pub fn from_data(data: Data, ty: &Type, first_type: &Type, second_type: &Type) -> Result<StackItem> {
        match data {
            Data::Pair(pair) => {
                let first = StackItem::from_data(pair.values[0].clone(), first_type)?;
                let second = StackItem::from_data(pair.values[1].clone(), second_type)?;
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
}