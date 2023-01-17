use std::fmt::Display;
use tezos_michelson::michelson::{
    data::Data,
    data,
    types::Type,
    types
};

use crate::{
    Result,
    types::{PairItem, StackItem},
    formatter::Formatter,
    err_mismatch,
    type_cast, typechecker::check_pair_len
};

impl PairItem {
    pub fn new(first: StackItem, second: StackItem) -> Self {
        Self(Box::new((first, second)))
    }

    pub fn from_items(mut items: Vec<StackItem>) -> Result<Self> {
        match items.len() {
            2 => Ok(Self::new(items.remove(0), items.remove(0))),
            n if n > 2 => {
                let leaf = items.remove(0);
                let node = Self::from_items(items)?;
                Ok(Self::new(leaf, node.into()))
            },
            i => err_mismatch!(">=2 args", i)
        }
    }

    pub fn from_data(data: Data, first_type: &Type, second_type: &Type) -> Result<StackItem> {
        match data {
            Data::Pair(mut pair) => {
                check_pair_len(pair.values.len())?;
                let first = StackItem::from_data(pair.values.remove(0), first_type)?;
                let second = StackItem::from_data(pair.values.remove(0), second_type)?;
                Ok(StackItem::Pair(Self::new(first, second)))
            },
            _ => err_mismatch!("Pair", data.format())
        }
    }
    
    pub fn into_data(self, ty: &Type) -> Result<Data> {
        let ty = type_cast!(ty, Pair);
        match ty.types.len() {
            2 => {},
            i => return err_mismatch!(">=2 args", i)
        }
        let first = self.0.0.into_data(&ty.types[0])?;
        let second = self.0.1.into_data(&ty.types[1])?;
        Ok(Data::Pair(data::pair(vec![first, second])))
    }

    pub fn into_items(self, arity: usize) -> Result<Vec<StackItem>> {
        match arity {
            2 => Ok(vec![self.0.0, self.0.1]),
            n if n > 2 => {
                let mut items = match self.0.1 {
                    StackItem::Pair(inner_pair) => inner_pair.into_items(arity - 1)?,
                    item => return err_mismatch!("PairItem (inner)", item)
                };
                items.insert(0, self.0.0);
                Ok(items)
            },
            i => err_mismatch!(">=2 args", i)
        }
    }

    pub fn unpair(self) -> (StackItem, StackItem) {
        (self.0.0, self.0.1)
    }

    pub fn get(&self, idx: usize) -> Result<StackItem> {
        match idx {
            0 => Ok(self.clone().into()),
            1 => Ok(self.0.0.clone()),
            2 => Ok(self.0.1.clone()),
            _ => match &self.0.1 {
                StackItem::Pair(inner_pair) => inner_pair.get(idx - 2),
                item => err_mismatch!("PairItem (inner)", item)
            }
        }
    }

    //        0
    //      /   \
    //     1     2
    //         /   \
    //        3     4
    //            /   \
    //           5    ...
    //                2n-2
    //              /      \
    //            2n-1      2n
    fn update_odd(&mut self, idx: usize, item: StackItem) -> Result<()> {
        match idx {
            0 => {
                self.0.0 = item;
                Ok(())
            },
            _ => match self.0.1.as_mut() {
                StackItem::Pair(right_pair) => right_pair.update_odd(idx - 1, item),
                _ => err_mismatch!(format!("PairItem (right {})", idx - 1), item)
            }
        }   
    }

    fn update_even(&mut self, idx: usize, item: StackItem) -> Result<()> {
        match idx {
            0 => match item {
                StackItem::Pair(root_pair) => {
                    *self = root_pair;
                    Ok(())
                },
                _ => err_mismatch!("PairItem (root)", item)
            },
            1 => {
                self.0.1 = item;
                Ok(())
            },
            _ => match self.0.1.as_mut() {
                StackItem::Pair(right_pair) => right_pair.update_even(idx - 1, item),
                _ => err_mismatch!(format!("PairItem (right {})", idx - 1), item)
            }
        }
    }

    pub fn update(&mut self, idx: usize, item: StackItem) -> Result<()> {
        match idx % 2 {
            0 => self.update_even(idx / 2, item),
            1 => self.update_odd((idx - 1) / 2, item),
            _ => unreachable!()
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
