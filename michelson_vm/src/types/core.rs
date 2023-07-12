// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use hex;
use std::fmt::Display;
use std::ops::{Add, BitAnd, BitOr, BitXor, Not};
use tezos_michelson::michelson::{
    data,
    data::Data,
    types,
    types::{ComparableType, Type},
};

use crate::{
    comparable_type_cast, err_mismatch,
    formatter::Formatter,
    types::{BoolItem, BytesItem, OptionItem, StackItem, StringItem, UnitItem},
    Result,
};

impl UnitItem {
    pub fn from_data(data: Data) -> Result<StackItem> {
        match data {
            Data::Unit(_) => Ok(StackItem::Unit(Self(()))),
            _ => err_mismatch!("Data::Unit", data.format()),
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        comparable_type_cast!(ty, Unit);
        Ok(Data::Unit(data::unit()))
    }
}

impl BoolItem {
    pub fn from_data(data: Data) -> Result<StackItem> {
        match data {
            Data::True(_) => return Ok(StackItem::Bool(true.into())),
            Data::False(_) => return Ok(StackItem::Bool(false.into())),
            _ => err_mismatch!("True or False", data.format()),
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        comparable_type_cast!(ty, Bool);
        match self.0 {
            true => Ok(Data::True(data::True)),
            false => Ok(Data::False(data::False)),
        }
    }

    pub fn is_true(&self) -> bool {
        self.0
    }
}

impl StringItem {
    pub fn from_data(data: Data) -> Result<StackItem> {
        match data {
            Data::String(val) => Ok(StackItem::String(Self(val.into_string()))),
            _ => err_mismatch!("String", data.format()),
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        comparable_type_cast!(ty, String);
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
    pub fn from_data(data: Data) -> Result<StackItem> {
        match data {
            Data::Bytes(val) => Ok(StackItem::Bytes(Self((&val).into()))),
            _ => err_mismatch!("Bytes", data.format()),
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        comparable_type_cast!(ty, Bytes);
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
