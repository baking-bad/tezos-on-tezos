// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use std::fmt::Display;
use tezos_michelson::michelson::{data::Data, types, types::Type};

use crate::{
    err_mismatch,
    formatter::Formatter,
    type_cast,
    types::list::{item_vec_into_seq, seq_into_item_vec},
    types::{SetItem, StackItem},
    Result,
};

impl SetItem {
    pub fn new(items: Vec<StackItem>, val_type: Type) -> Self {
        let mut items = items;
        items.sort_unstable();
        Self {
            outer_value: items,
            inner_type: val_type,
        }
    }

    pub fn from_data(data: Data, val_type: &Type) -> Result<StackItem> {
        match data {
            Data::Sequence(seq) => {
                let items = seq_into_item_vec(seq, val_type)?;
                // TODO: ensure no duplicates
                Ok(StackItem::Set(Self::new(items, val_type.clone())))
            }
            _ => err_mismatch!("Sequence", data.format()),
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        let ty = type_cast!(ty, Set);
        item_vec_into_seq(self.outer_value, &self.inner_type, &ty.r#type)
    }

    pub fn into_elements(self) -> (Vec<StackItem>, Type) {
        (self.outer_value, self.inner_type)
    }

    pub fn get_type(&self) -> Result<Type> {
        match &self.inner_type {
            Type::Comparable(ty) => Ok(types::set(ty.clone().into())),
            ty => err_mismatch!("ComparableType", ty.format()),
        }
    }

    pub fn contains(&self, key: &StackItem) -> Result<bool> {
        key.type_check(&self.inner_type)?;
        Ok(self.outer_value.contains(key))
    }

    pub fn update(&mut self, key: StackItem, val: bool) -> Result<()> {
        key.type_check(&self.inner_type)?;
        match self.outer_value.binary_search(&key) {
            Ok(pos) => {
                if !val {
                    self.outer_value.remove(pos);
                }
            }
            Err(pos) => {
                if val {
                    self.outer_value.insert(pos, key);
                }
            }
        }
        Ok(())
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
        if self.len() == 0 {
            f.write_fmt(format_args!(" /* {} */ ", self.inner_type.format()))?;
        }
        f.write_str("}")
    }
}
