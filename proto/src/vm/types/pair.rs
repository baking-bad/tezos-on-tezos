use std::fmt::Display;
use tezos_michelson::michelson::{
    data::Data,
    data,
    types::Type,
    types
};

use crate::{
    Result,
    error::InterpreterError,
    vm::types::{PairItem, StackItem},
    err_type,
};

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
            n => Err(InterpreterError::InvalidArity { expected: 2, found: n }.into())
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

impl Display for PairItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({} * {})", self.0.0, self.0.1))
    }
}
