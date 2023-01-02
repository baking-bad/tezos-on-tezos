use std::fmt::Display;
use tezos_michelson::michelson::{
    data::{Data, Sequence},
    types::Type,
    types,
    data
};

use crate::{
    Result,
    vm::types::{MapItem, StackItem, PairItem, OptionItem},
    vm::typechecker::check_types_equal,
    err_type,
};

impl MapItem {
    pub fn new(items: Vec<(StackItem, StackItem)>, key_type: Type, val_type: Type) -> Self {
        Self { outer_value: items, inner_type: (key_type, val_type) }
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
                let ty: types::Map = types::map(key_type.clone(), val_type.clone());
                return err_type!(ty, element)
            }
        }
        return Ok(Self::new(items, key_type, val_type))
    }

    pub fn from_data(data: Data, ty: &Type, key_type: &Type, val_type: &Type) -> Result<StackItem> {
        match data {
            Data::Sequence(sequence) => {
                let elements = sequence.into_values();
                let mut items: Vec<(StackItem, StackItem)> = Vec::with_capacity(elements.len());
                for element in elements {
                    if let Data::Elt(elt) = element {
                        let key = StackItem::from_data(*elt.key, key_type)?;
                        let val = StackItem::from_data(*elt.value, val_type)?;
                        items.push((key, val));
                    } else {
                        let ty: types::Map = types::map(key_type.clone(), val_type.clone());
                        return err_type!(ty, element)
                    }
                }
                Ok(StackItem::Map(Self::new(items, key_type.clone(), val_type.clone())))
            },
            _ => err_type!(ty, data)
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        match ty {
            Type::Map(map_ty) => {
                if self.outer_value.is_empty() {
                    check_types_equal(&map_ty.key_type, &self.inner_type.0)?;
                    check_types_equal(&map_ty.value_type, &self.inner_type.1)?;
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
            },
            _ => err_type!(ty, self)
        }
    }

    pub fn unwrap(self) -> (Vec<StackItem>, (Type, Type)) {
        let mut elements: Vec<StackItem> = Vec::with_capacity(self.outer_value.len());
        for (key_item, val_item) in self.outer_value {
            elements.push(PairItem::new(key_item, val_item).into());
        }
        (elements, self.inner_type)
    }

    pub fn get_type(&self) -> Type {
        let (kty, vty) = self.inner_type.clone();
        types::map(kty, vty)
    }

    pub fn get_keys(&self) -> Vec<StackItem> {
        self.outer_value.iter().map(|(k, _)| k.clone()).collect()
    }

    pub fn get(&self, key: &StackItem) -> Result<OptionItem> {
        key.type_check(&self.inner_type.0)?;
        match self.outer_value.iter().find(|(k, _)| k == key) {
            Some((_, val)) => OptionItem::some(val.clone()),
            None => Ok(OptionItem::none(&self.inner_type.0))
        }
    }

    pub fn update(self, key: StackItem, val: Option<StackItem>) -> Result<(Self, OptionItem)> {
        let (key_type, val_type) = self.inner_type;
        let mut items = self.outer_value;
        let mut old_val: Option<Box<StackItem>> = None;
        key.type_check(&key_type)?;
        match items.binary_search_by(|(k, _)| k.cmp(&key)) {
            Ok(pos) => if val.is_none() {
                let (_, v) = items.remove(pos);
                old_val = Some(Box::new(v));
            },
            Err(pos) => if let Some(val) = val {
                items.insert(pos, (key, val));
            }
        }
        let old = OptionItem::new(old_val, &val_type);
        let map = Self::new(items, key_type, val_type);
        Ok((map, old))
    }

    pub fn contains(&self, key: &StackItem) -> Result<bool> {
        key.type_check(&self.inner_type.0)?;
        Ok(self.outer_value.iter().find(|(k, _)| k == key).is_some())
    }

    pub fn len(&self) -> usize {
        self.outer_value.len()
    }
}

impl Display for MapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{")?;
        for (i, (key, val)) in self.outer_value.iter().enumerate() {
            if i != 0 {
                f.write_str(", ")?;
            }
            f.write_fmt(format_args!("{}: {}", key, val))?;
        }
        f.write_str("}")
    }
}