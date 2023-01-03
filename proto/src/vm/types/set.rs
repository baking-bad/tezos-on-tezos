use std::fmt::Display;
use tezos_michelson::michelson::{
    data::Data,
    types::Type,
    types
};

use crate::{
    Result,
    vm::types::{SetItem, StackItem},
    vm::types::list::{seq_into_item_vec, item_vec_into_seq},
    err_type,
};

impl SetItem {
    pub fn new(items: Vec<StackItem>, val_type: Type) -> Self {
        Self { outer_value: items, inner_type: val_type }
    }

    pub fn from_data(data: Data, ty: &Type, val_type: &Type) -> Result<StackItem> {        
        match data {
            Data::Sequence(seq) => {
                let items = seq_into_item_vec(seq, val_type)?;
                // TODO: ensure no duplicates
                Ok(StackItem::Set(Self::new(items, val_type.clone())))
            },
            _ => err_type!(ty, data)
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        match ty {
            Type::Set(set_ty) => {
                item_vec_into_seq(self.outer_value, &self.inner_type, &set_ty.r#type.clone().into())
            },
            _ => err_type!(ty, self)
        }
    }

    pub fn unwrap(self) -> (Vec<StackItem>, Type) {
        (self.outer_value, self.inner_type)
    }

    pub fn get_type(&self) -> Result<Type> {
        match &self.inner_type {
            Type::Comparable(ty) => Ok(types::set(ty.clone())),
            ty => err_type!("ComparableType", ty)
        }
    }

    pub fn contains(&self, key: &StackItem) -> Result<bool> {
        key.type_check(&self.inner_type)?;
        Ok(self.outer_value.contains(key))
    }

    pub fn update(self, key: StackItem, val: bool) -> Result<Self> {
        key.type_check(&self.inner_type)?;
        let (mut items, val_type) = self.unwrap();
        match items.binary_search(&key) {
            Ok(pos) => if !val {
                items.remove(pos);
            },
            Err(pos) => if val {
                items.insert(pos, key);
            }
        }
        Ok(Self::new(items, val_type))
    }

    pub fn len(&self) -> usize {
        self.outer_value.len()
    }
}

impl PartialEq for SetItem {
    fn eq(&self, other: &Self) -> bool {
        // For testing purposes
        self.outer_value == other.outer_value
    }
}

impl Display for SetItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{")?;
        for (i, item) in self.outer_value.iter().enumerate() {
            if i != 0 {
                f.write_str(", ")?;
            }
            item.fmt(f)?;
        }
        f.write_str("}")
    }
}