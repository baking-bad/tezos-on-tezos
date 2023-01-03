use std::fmt::Display;
use tezos_michelson::michelson::{
    data::Data,
    types::Type,
    types,
    data
};

use crate::{
    Result,
    Error,
    types::{ListItem, StackItem},
    typechecker::check_types_equal,
    err_type,
};

pub fn seq_into_item_vec(sequence: data::Sequence, val_type: &Type) -> Result<Vec<StackItem>> {
    let values = sequence.into_values();
    let mut items: Vec<StackItem> = Vec::with_capacity(values.len());
    for value in values {
        items.push(StackItem::from_data(value, val_type)?);
    }
    Ok(items)
}

pub fn item_vec_into_seq(items: Vec<StackItem>, inner_type: &Type, val_type: &Type) -> Result<Data> {
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

    pub fn from_data(data: Data, val_type: &Type) -> Result<StackItem> {
        match data {
            Data::Sequence(seq) => {
                let items = seq_into_item_vec(seq, &val_type)?;
                Ok(StackItem::List(Self::new(items, val_type.to_owned())))
            },
            _ => err_type!("Data::Sequence", data)
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
            Err(Error::GeneralOverflow.into())
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

impl PartialEq for ListItem {
    fn eq(&self, other: &Self) -> bool {
        // For testing purposes
        self.outer_value == other.outer_value
    }
}

impl Display for ListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        for (i, item) in self.outer_value.iter().enumerate() {
            if i != 0 {
                f.write_str(", ")?;
            }
            item.fmt(f)?;
        }
        f.write_str("]")
    }
}