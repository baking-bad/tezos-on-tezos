use std::fmt::Display;
use std::ops::{Add, Sub};
use ibig::IBig;
use chrono::{DateTime, NaiveDateTime, Utc};
use tezos_michelson::michelson::{
    data::Data, data,
    types::{Type, ComparableType}
};

use crate::{
    Result,
    vm::types::{IntItem, TimestampItem, StackItem},
    err_type,
    type_check_fn_comparable,
};

impl TimestampItem {
    type_check_fn_comparable!(Timestamp);

    pub fn new(value: i64) -> Result<Self> {
        Ok(Self(value))  // TODO: check non-negative
    }

    pub fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        let timestamp = match data {
            Data::String(val) => DateTime::parse_from_rfc3339(val.to_str())?.timestamp(),
            Data::Int(val) => val.to_integer()?,
            _ => return err_type!(ty, data)
        };
        Ok(StackItem::Timestamp(Self::new(timestamp)?))
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

impl Display for TimestampItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match NaiveDateTime::from_timestamp_opt(self.0, 0) {
            Some(dt) => DateTime::<Utc>::from_utc(dt, Utc).to_rfc3339(),
            None => format!("{}+0", self.0)
        };
        f.write_str(str.as_str())
    }
}

impl Add<IntItem> for TimestampItem {
    type Output = Result<TimestampItem>;

    fn add(self, rhs: IntItem) -> Self::Output {
        let delta: i64 = rhs.0.try_into()?;
        Ok(TimestampItem(self.0 + delta))
    }
}

impl Sub<IntItem> for TimestampItem {
    type Output = Result<TimestampItem>;

    fn sub(self, rhs: IntItem) -> Self::Output {
        let b: i64 = rhs.0.try_into()?;
        TimestampItem::new(self.0 - b)
    }
}

impl Sub<TimestampItem> for TimestampItem {
    type Output = IntItem;

    fn sub(self, rhs: TimestampItem) -> Self::Output {
        IntItem(IBig::from(self.0 - rhs.0))
    }
}