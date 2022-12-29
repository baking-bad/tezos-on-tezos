use ibig::{IBig, UBig};
use chrono::{DateTime, NaiveDateTime, Utc};
use tezos_michelson::michelson::{
    data::Data, data,
    types::{Type, ComparableType}
};

use crate::{
    Result, Error,
    vm::types::{IntItem, NatItem, MutezItem, TimestampItem, StackItem},
    err_type,
    type_check_fn_comparable,
};

pub fn assert_non_negative(value: data::Int, ty: &Type) -> Result<i64> {
    let int: i64 = value.to_integer()?;
    if int >= 0 { Ok(int) } else { err_type!(ty, value) }
}

impl IntItem {
    type_check_fn_comparable!(Int);

    pub fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        match data {
            Data::Int(val) => Ok(StackItem::Int(IBig::from_str_radix(val.to_str(), 10)?.into())),
            _ => err_type!(ty, data)
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        self.type_check(ty)?;
        Ok(Data::Int(self.0.into()))
    }
}

impl NatItem {
    type_check_fn_comparable!(Nat);

    pub fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        match data {
            Data::Int(val) => Ok(StackItem::Nat(UBig::from_str_radix(val.to_str(), 10)?.into())),
            _ => err_type!(ty, data)
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        self.type_check(ty)?;
        let int: IBig = self.0.into();
        Ok(Data::Int(int.into()))
    }
}

impl TimestampItem {
    type_check_fn_comparable!(Timestamp);

    pub fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        let timestamp = match data {
            Data::String(val) => DateTime::parse_from_rfc3339(val.to_str())?.timestamp(),
            Data::Int(val) => assert_non_negative(val, ty)?,
            _ => return err_type!(ty, data)
        };
        Ok(StackItem::Timestamp(timestamp.into()))
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        self.type_check(ty)?;
        let dt = match NaiveDateTime::from_timestamp_opt(self.0, 0) {
            Some(dt) => DateTime::<Utc>::from_utc(dt, Utc),
            None => return err_type!(ty, self)
        };
        Ok(Data::String(data::String::from_string(dt.to_rfc3339())?))
    }
}

impl MutezItem {
    type_check_fn_comparable!(Mutez);

    pub fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        match data {
            Data::Int(val) => Ok(StackItem::Mutez(assert_non_negative(val, ty)?.into())),
            _ => err_type!(ty, data)
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        self.type_check(ty)?;
        Ok(Data::Int(self.0.into()))
    }
}