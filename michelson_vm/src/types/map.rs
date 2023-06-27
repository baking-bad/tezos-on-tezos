// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use std::fmt::Display;
use tezos_michelson::michelson::{
    data,
    data::{Data, Sequence},
    types,
    types::Type,
};

use crate::{
    err_mismatch,
    formatter::Formatter,
    type_cast,
    typechecker::check_types_equal,
    types::{MapItem, OptionItem, PairItem, StackItem},
    Result,
};

impl MapItem {
    pub fn new(items: Vec<(StackItem, StackItem)>, key_type: Type, val_type: Type) -> Self {
        let mut items = items;
        items.sort_unstable_by(|(k1, _), (k2, _)| k1.cmp(&k2));
        Self {
            outer_value: items,
            inner_type: (key_type, val_type),
        }
    }

    pub fn from_sequence(sequence: Sequence, key_type: Type, val_type: Type) -> Result<Self> {
        let elements = sequence.into_values();
        let mut items: Vec<(StackItem, StackItem)> = Vec::with_capacity(elements.len());
        for element in elements {
            if let Data::Elt(elt) = element {
                let key = StackItem::from_data(*elt.key, &key_type)?;
                let val = StackItem::from_data(*elt.value, &val_type)?;
                items.push((key, val));
            } else {
                return err_mismatch!(
                    format!("Elt {} => {}", key_type.format(), val_type.format()),
                    element.format()
                );
            }
        }
        return Ok(Self::new(items, key_type, val_type));
    }

    pub fn from_elt_map(map: data::Map, key_type: Type, val_type: Type) -> Result<Self> {
        let elements = map.into_values();
        let mut items: Vec<(StackItem, StackItem)> = Vec::with_capacity(elements.len());
        for elt in elements {
            let key = StackItem::from_data(*elt.key, &key_type)?;
            let val = StackItem::from_data(*elt.value, &val_type)?;
            items.push((key, val));
        }
        return Ok(Self::new(items, key_type, val_type));
    }

    pub fn from_data(data: Data, key_type: &Type, val_type: &Type) -> Result<StackItem> {
        match data {
            Data::Sequence(sequence) => {
                let item = Self::from_sequence(sequence, key_type.clone(), val_type.clone())?;
                Ok(item.into())
            }
            Data::Map(elt_map) => {
                let item = Self::from_elt_map(elt_map, key_type.clone(), val_type.clone())?;
                Ok(item.into())
            }
            _ => err_mismatch!("Sequence or Map", data.format()),
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        let ty = type_cast!(ty, Map);
        if self.outer_value.is_empty() {
            check_types_equal(&ty.key_type, &self.inner_type.0)?;
            check_types_equal(&ty.value_type, &self.inner_type.1)?;
            Ok(Data::Sequence(data::sequence(vec![])))
        } else {
            let mut elements: Vec<Data> = Vec::with_capacity(self.outer_value.len());
            for (key_item, val_item) in self.outer_value {
                let key = key_item.into_data(&self.inner_type.0)?;
                let value = val_item.into_data(&self.inner_type.1)?;
                elements.push(Data::Elt(data::elt(key, value)));
            }
            Ok(Data::Sequence(data::sequence(elements)))
        }
    }

    pub fn into_pairs(self) -> (Vec<StackItem>, (Type, Type)) {
        let mut elements: Vec<StackItem> = Vec::with_capacity(self.outer_value.len());
        for (key_item, val_item) in self.outer_value {
            elements.push(PairItem::new(key_item, val_item).into());
        }
        (elements, self.inner_type)
    }

    pub fn into_elements(self) -> (Vec<(StackItem, StackItem)>, (Type, Type)) {
        (self.outer_value, self.inner_type)
    }

    pub fn get_type(&self) -> Result<Type> {
        let (kty, vty) = self.inner_type.clone();
        Ok(types::map(kty, vty))
    }

    pub fn get_keys(&self) -> Vec<StackItem> {
        self.outer_value.iter().map(|(k, _)| k.clone()).collect()
    }

    pub fn get(&self, key: &StackItem) -> Result<OptionItem> {
        key.type_check(&self.inner_type.0)?;
        match self.outer_value.iter().find(|(k, _)| k == key) {
            Some((_, val)) => Ok(OptionItem::Some(Box::new(val.clone()))),
            None => Ok(OptionItem::None(self.inner_type.0.clone())),
        }
    }

    pub fn update(&mut self, key: StackItem, val: Option<StackItem>) -> Result<OptionItem> {
        key.type_check(&self.inner_type.0)?;
        match self.outer_value.binary_search_by(|(k, _)| k.cmp(&key)) {
            Ok(pos) => {
                let (_, v) = self.outer_value.remove(pos);
                if let Some(val) = val {
                    self.outer_value.insert(pos, (key, val));
                }
                Ok(OptionItem::some(v))
            }
            Err(pos) => {
                if let Some(val) = val {
                    self.outer_value.insert(pos, (key, val));
                }
                Ok(OptionItem::none(&self.inner_type.1))
            }
        }
    }

    pub fn contains(&self, key: &StackItem) -> Result<bool> {
        key.type_check(&self.inner_type.0)?;
        Ok(self.outer_value.iter().find(|(k, _)| k == key).is_some())
    }

    pub fn len(&self) -> usize {
        self.outer_value.len()
    }
}

impl PartialEq for MapItem {
    fn eq(&self, other: &Self) -> bool {
        // For testing purposes
        self.outer_value == other.outer_value
    }
}

impl Display for MapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{ ")?;
        for (i, (key, val)) in self.outer_value.iter().enumerate() {
            if i != 0 {
                f.write_str(", ")?;
            }
            f.write_fmt(format_args!("{} => {}", key, val))?;
        }
        if self.len() == 0 {
            f.write_fmt(format_args!(
                " /* {} => {} */ ",
                self.inner_type.0.format(),
                self.inner_type.1.format()
            ))?;
        }
        f.write_str(" }")
    }
}
