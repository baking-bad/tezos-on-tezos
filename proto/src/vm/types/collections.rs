use tezos_michelson::michelson::{
    data::Data,
    data,
    types::Type,
    types
};

use crate::{
    Result, Error,
    vm::types::{ListItem, SetItem, MapItem, BigMapItem, StackItem, PairItem, BigMapPtr, OptionItem},
    vm::typechecker::check_types_equal,
    err_type,
};

fn seq_into_item_vec(sequence: data::Sequence, val_type: &Type) -> Result<Vec<StackItem>> {
    let values = sequence.into_values();
    let mut items: Vec<StackItem> = Vec::with_capacity(values.len());
    for value in values {
        items.push(StackItem::from_data(value, val_type)?);
    }
    Ok(items)
}

fn item_vec_into_seq(items: Vec<StackItem>, inner_type: &Type, val_type: &Type) -> Result<Data> {
    if items.is_empty() {
        check_types_equal(val_type, inner_type)?;
        Ok(Data::Sequence(data::sequence(vec![])))
    } else {
        let mut values: Vec<Data> = Vec::with_capacity(items.len());
        for item in items {
            values.push(item.into_data(&inner_type)?);
        }
        Ok(Data::Sequence(data::sequence(values)))
    }
}

impl ListItem {
    pub fn new(items: Vec<StackItem>, val_type: Type) -> Self {
        Self { outer_value: items, inner_type: val_type }
    }

    pub fn from_data(data: Data, ty: &Type, val_type: &Type) -> Result<StackItem> {
        match data {
            Data::Sequence(seq) => {
                let items = seq_into_item_vec(seq, &val_type)?;
                Ok(StackItem::List(Self::new(items, val_type.to_owned())))
            },
            _ => err_type!(ty, data)
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        match ty {
            Type::List(list_ty) => item_vec_into_seq(self.outer_value, &self.inner_type, &list_ty.r#type),
            _ => err_type!(ty, self)
        }
    }

    pub fn unwrap(self) -> (Vec<StackItem>, Type) {
        (self.outer_value, self.inner_type)
    }

    pub fn get_type(&self) -> Type {
        types::list(self.inner_type.clone())
    }

    pub fn split_head(self) -> Result<(StackItem, ListItem)> {
        if self.outer_value.len() > 0 {
            let (mut items, val_type) = self.unwrap();
            let head = items.remove(0);
            Ok((head, Self::new(items, val_type)))
        } else {
            Err(Error::ListOutOfBounds)
        }
    }

    pub fn prepend(self, item: StackItem) -> Result<ListItem> {
        item.type_check(&self.inner_type)?;
        let (mut items, val_type) = self.unwrap();
        items.insert(0, item);
        Ok(Self::new(items, val_type))
    }

    pub fn len(&self) -> usize {
        self.outer_value.len()
    }
}

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

fn seq_into_map_item(sequence: data::Sequence, key_type: &Type, val_type: &Type) -> Result<MapItem> {
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
    return Ok(MapItem::new(items, key_type.clone(), val_type.clone()))
}

fn map_item_into_seq(map_item: MapItem, map_ty: &types::Map) -> Result<Data> {
    if map_item.outer_value.is_empty() {
        check_types_equal(&map_ty.key_type, &map_item.inner_type.0)?;
        check_types_equal(&map_ty.value_type, &map_item.inner_type.1)?;
        Ok(Data::Sequence(data::sequence(vec![])))
    } else {
        let mut elements: Vec<Data> = Vec::with_capacity(map_item.outer_value.len());
        for (key_item, val_item) in map_item.outer_value {
            let key = key_item.into_data(&map_item.inner_type.0)?;
            let value = val_item.into_data(&map_item.inner_type.1)?;
            elements.push(Data::Elt(data::elt(key, value)));
        }
        Ok(Data::Sequence(data::sequence(elements)))
    }
}

impl MapItem {
    pub fn new(items: Vec<(StackItem, StackItem)>, key_type: Type, val_type: Type) -> Self {
        Self { outer_value: items, inner_type: (key_type, val_type) }
    }

    pub fn from_data(data: Data, ty: &Type, key_type: &Type, val_type: &Type) -> Result<StackItem> {
        match data {
            Data::Sequence(sequence) => Ok(StackItem::Map(seq_into_map_item(sequence, key_type, val_type)?)),
            _ => err_type!(ty, data)
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        match ty {
            Type::Map(map_ty) => map_item_into_seq(self, map_ty),
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

impl BigMapItem {
    pub fn from_data(data: Data, ty: &Type, key_type: &Type, val_type: &Type) -> Result<StackItem> {
        match data {
            Data::Int(ptr) => {
                let ptr = BigMapPtr { value: ptr.to_integer()?, outer_type: ty.clone() };
                Ok(StackItem::BigMap(Self::Ptr(ptr)))
            },
            Data::Sequence(sequence) => {
                let map_item = seq_into_map_item(sequence, key_type, val_type)?;
                Ok(StackItem::BigMap(Self::Map(map_item)))
            },
            _ => err_type!(ty, data)
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        if let Type::BigMap(_) = ty {
            return match self {
                Self::Ptr(ptr) => {
                    check_types_equal(ty, &ptr.outer_type)?;
                    Ok(Data::Int(data::int(ptr.value)))
                },
                Self::Map(_) => err_type!(ty, self)  // NOTE: not supported
            }
        }
        err_type!(ty, self)
    }

    pub fn get_type(&self) -> Type {
        match self {
            Self::Ptr(ptr) => ptr.outer_type.clone(),
            Self::Map(map) => map.get_type()
        }
    }
}
