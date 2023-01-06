use std::ops::{BitOr, BitXor, BitAnd, Not, Add};
use std::fmt::Display;
use tezos_michelson::michelson::{
    data::Data,
    data,
    types::{Type, ComparableType},
    types
};
use hex;

use crate::{
    Result,
    types::{UnitItem, BoolItem, StringItem, BytesItem, StackItem, OptionItem},
    err_type,
    type_check_fn_comparable
};

impl UnitItem {
    type_check_fn_comparable!(Unit);

    pub fn from_data(data: Data) -> Result<StackItem> {
        match data {
            Data::Unit(_) => Ok(StackItem::Unit(Self(()))),
            _ => err_type!("Data::Unit", data)
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        self.type_check(ty)?;
        Ok(Data::Unit(data::unit()))
    }
}

impl BoolItem {
    type_check_fn_comparable!(Bool);

    pub fn from_data(data: Data) -> Result<StackItem> {
        match data {
            Data::True(_) => return Ok(StackItem::Bool(true.into())),
            Data::False(_) => return Ok(StackItem::Bool(false.into())),
            _ => err_type!("Data::True or Data::False", data)
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

    pub fn from_data(data: Data) -> Result<StackItem> {
        match data {
            Data::String(val) => Ok(StackItem::String(Self(val.into_string()))),
            _ => err_type!("Data::String", data)
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
        if self.len() > 0 && start <= end && end <= self.len() {
            let item = Self(self.0[start..end].to_string());
            OptionItem::some(item.into())
        } else {
            OptionItem::None(types::string())
        }
    }
}

impl BytesItem {
    type_check_fn_comparable!(Bytes);

    pub fn from_data(data: Data) -> Result<StackItem> {
        match data {
            Data::Bytes(val) => Ok(StackItem::Bytes(Self((&val).into()))),
            _ => err_type!("Data::Bytes", data)
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
        if self.len() > 0 && start <= end && end <= self.len() {
            let item = Self(self.0[start..end].to_vec());
            OptionItem::some(item.into())
        } else {
            OptionItem::None(types::bytes())
        }
    }
}

impl Display for UnitItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Unit")
    }
}

impl Display for BoolItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(if self.0 { "True" } else { "False" })
    }
}

impl Display for StringItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("\"{}\"", self.0))
    }
}

impl Display for BytesItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(hex::encode(self.0.clone()).as_str())
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