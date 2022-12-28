use tezos_michelson::michelson::{
    data::Data, data,
    types::{Type, ComparableType}
};

use crate::{
    Result, Error,
    vm::types::{UnitItem, BoolItem, StringItem, BytesItem, StackItem},
    err_type,
    type_check_comparable
};

impl UnitItem {
    type_check_comparable!(Unit);

    pub fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        match data {
            Data::Unit(_) => Ok(StackItem::Unit(Self(()))),
            _ => err_type!(ty, data)
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        self.type_check(ty)?;
        Ok(Data::Unit(data::unit()))
    }
}

impl BoolItem {
    type_check_comparable!(Bool);

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
}

impl StringItem {
    type_check_comparable!(String);

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
}

impl BytesItem {
    type_check_comparable!(Bytes);

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
}