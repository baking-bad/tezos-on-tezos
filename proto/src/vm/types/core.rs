use std::ops::{BitOr, BitXor, BitAnd, Not, Add};
use tezos_michelson::michelson::{
    data::Data,
    data,
    types::{Type, ComparableType},
    types
};

use crate::{
    Result, Error,
    vm::types::{BoolItem, StringItem, BytesItem, StackItem, OptionItem},
    err_type,
    type_check_fn_comparable
};

impl BoolItem {
    type_check_fn_comparable!(Bool);

    pub fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        match data {
            Data::True(_) => return Ok(StackItem::Bool(true.into())),
            Data::False(_) => return Ok(StackItem::Bool(false.into())),
            _ => err_type!(ty, data)
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        self.type_check(ty)?;
        match self.0 {
            true => Ok(Data::True(data::True)),
            false => Ok(Data::False(data::False))
        }
    }

    pub fn is_true(&self) -> bool {
        self.0
    }
}

impl StringItem {
    type_check_fn_comparable!(String);

    pub fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        match data {
            Data::String(val) => Ok(StackItem::String(Self(val.into_string()))),
            _ => err_type!(ty, data)
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        self.type_check(ty)?;
        Ok(Data::String(data::String::from_string(self.0)?))
    }

    pub fn unwrap(self) -> String {
        self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn slice(self, start: usize, end: usize) -> OptionItem {
        let outer_value = if start <= end && end <= self.len() {
            let item = Self(self.0[start..end].to_string());
            Some(Box::new(item.into()))
        } else {
            None
        };
        OptionItem { outer_value, inner_type: types::string() }
    }
}

impl BytesItem {
    type_check_fn_comparable!(Bytes);

    pub fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        match data {
            Data::Bytes(val) => Ok(StackItem::Bytes(Self((&val).into()))),
            _ => err_type!(ty, data)
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        self.type_check(ty)?;
        Ok(Data::Bytes(data::bytes(self.0)))
    }

    pub fn unwrap(self) -> Vec<u8> {
        self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn slice(self, start: usize, end: usize) -> OptionItem {
        let outer_value = if start <= end && end <= self.len() {
            let item = Self(self.0[start..end].to_vec());
            Some(Box::new(item.into()))
        } else {
            None
        };
        OptionItem { outer_value, inner_type: types::bytes() }
    }
}

impl BitOr<BoolItem> for BoolItem {
    type Output = BoolItem;

    fn bitor(self, rhs: BoolItem) -> Self::Output {
        BoolItem(self.0 | rhs.0)
    }
}

impl BitXor<BoolItem> for BoolItem {
    type Output = BoolItem;

    fn bitxor(self, rhs: BoolItem) -> Self::Output {
        BoolItem(self.0 ^ rhs.0)
    }
}

impl BitAnd<BoolItem> for BoolItem {
    type Output = BoolItem;

    fn bitand(self, rhs: BoolItem) -> Self::Output {
        BoolItem(self.0 & rhs.0)
    }
}

impl Not for BoolItem {
    type Output = BoolItem;

    fn not(self) -> Self::Output {
        BoolItem(!self.0)
    }
}

impl Add<StringItem> for StringItem {
    type Output = StringItem;

    fn add(self, rhs: StringItem) -> Self::Output {
        StringItem([self.0, rhs.0].concat())
    }
}

impl Add<BytesItem> for BytesItem {
    type Output = BytesItem;

    fn add(self, rhs: BytesItem) -> Self::Output {
        BytesItem([self.0, rhs.0].concat())
    }
}