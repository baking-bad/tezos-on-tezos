use std::fmt::Display;
use std::ops::{Add, Sub};
use ibig::IBig;
use chrono::{DateTime, NaiveDateTime, Utc, SecondsFormat};
use tezos_michelson::michelson::{
    data::Data, data,
    types::{Type, ComparableType}
};

use crate::{
    Result,
    types::{IntItem, TimestampItem, StackItem},
    err_type,
    comparable_type_cast
};

impl TimestampItem {
    pub fn new(value: i64) -> Result<Self> {
        Ok(Self(value))
    }

    pub fn from_data(data: Data) -> Result<StackItem> {
        let timestamp = match data {
            Data::String(val) => DateTime::parse_from_rfc3339(val.to_str())?.timestamp(),
            Data::Int(val) => val.to_integer()?,
            _ => return err_type!("Data::String or Data::Int", data)
        };
        Ok(StackItem::Timestamp(Self::new(timestamp)?))
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        comparable_type_cast!(ty, Timestamp);
        let dt = match NaiveDateTime::from_timestamp_opt(self.0, 0) {
            Some(dt) => DateTime::<Utc>::from_utc(dt, Utc),
            None => return err_type!(ty, self)
        };
        let string = dt.to_rfc3339_opts(SecondsFormat::Secs, true);
        Ok(Data::String(data::String::from_string(string)?))
    }
}

impl Display for TimestampItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match NaiveDateTime::from_timestamp_opt(self.0, 0) {
            Some(dt) => DateTime::<Utc>::from_utc(dt, Utc)
                .to_rfc3339_opts(SecondsFormat::Secs, true),
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