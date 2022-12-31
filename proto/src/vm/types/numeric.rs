use std::ops::{Add, Div, Shl, Shr, Mul, Neg, Sub, BitOr, BitXor, BitAnd, Not};
use ibig::{IBig, UBig};
use ibig::ops::{Abs, UnsignedAbs};
use chrono::{DateTime, NaiveDateTime, Utc};
use tezos_michelson::michelson::{
    data::Data, data,
    types::{Type, ComparableType, int, nat, mutez, pair}
};

use crate::{
    Result, Error,
    vm::types::{IntItem, NatItem, MutezItem, TimestampItem, StackItem, OptionItem, PairItem},
    err_type,
    type_check_fn_comparable,
};

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

    pub fn abs(self) -> Result<NatItem> {
        Ok(NatItem(self.0.unsigned_abs()))
    }

    pub fn nat(self) -> Result<OptionItem> {
        let outer_value = if self.0 > 0.into() {
            let nat = NatItem(self.0.try_into()?);
            Some(Box::new(StackItem::Nat(nat)))
        } else {
            None
        };
        Ok(OptionItem { outer_value, inner_type: nat() })
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

    pub fn int(self) -> IntItem {
        IntItem(IBig::from(self.0))
    }
}

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

impl MutezItem {
    type_check_fn_comparable!(Mutez);

    pub fn new(value: i64) -> Result<Self> {
        Ok(Self(value)) // TODO: check overflow
    }

    pub fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        match data {
            Data::Int(val) => Ok(StackItem::Mutez(Self::new(val.to_integer()?)?)),
            _ => err_type!(ty, data)
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        self.type_check(ty)?;
        Ok(Data::Int(self.0.into()))
    }
}

impl Add<IntItem> for IntItem {
    type Output = IntItem;

    fn add(self, rhs: IntItem) -> Self::Output {
        IntItem(self.0 + rhs.0)
    }
}

impl Add<NatItem> for NatItem {
    type Output = NatItem;

    fn add(self, rhs: Self) -> Self::Output {
        NatItem(self.0 + rhs.0)
    }
}

impl Add<IntItem> for NatItem {
    type Output = IntItem;

    fn add(self, rhs: IntItem) -> Self::Output {
        IntItem(IBig::from(self.0) + rhs.0)
    }
}

impl Add<IntItem> for TimestampItem {
    type Output = Result<TimestampItem>;

    fn add(self, rhs: IntItem) -> Self::Output {
        let delta: i64 = rhs.0.try_into()?;
        Ok(TimestampItem(self.0 + delta))
    }
}

impl Add<MutezItem> for MutezItem {
    type Output = Result<MutezItem>;

    fn add(self, rhs: MutezItem) -> Self::Output {
        Ok(MutezItem(self.0 + rhs.0))  // TODO: check overflow
    }
}

impl Div<NatItem> for NatItem {
    type Output = OptionItem;

    fn div(self, rhs: NatItem) -> Self::Output {
        let inner_type = pair(vec![nat(), nat()]);
        let outer_value = if rhs.0 == 0u8.into() {
            None
        } else {
            let (q, r) = (&self.0 / &rhs.0, &self.0 % &rhs.0);
            Some(Box::new(PairItem::new(
                NatItem(q).into(), 
                NatItem(r).into()
            ).into()))
        };
        OptionItem { outer_value, inner_type }
    }
}

impl Div<IntItem> for IntItem {
    type Output = OptionItem;

    fn div(self, rhs: IntItem) -> Self::Output {
        let inner_type = pair(vec![int(), nat()]);
        let outer_value = if rhs.0 == 0i8.into() {
            None
        } else {
            let (mut q, mut r) = (&self.0 / &rhs.0, &self.0 % &rhs.0);
            if r < 0i8.into() {
                r += rhs.0.abs();
                q += 1
            }
            Some(Box::new(PairItem::new(
                IntItem(q).into(), 
                NatItem(UBig::try_from(r).expect("positive remainder")).into()
            ).into()))
        };
        OptionItem { outer_value, inner_type }
    }
}

impl Div<NatItem> for MutezItem {
    type Output = Result<OptionItem>;

    fn div(self, rhs: NatItem) -> Self::Output {
        let inner_type = pair(vec![mutez(), mutez()]);
        let outer_value = if rhs.0 == 0u8.into() {
            None
        } else {
            let (a, b) = (IBig::from(self.0), IBig::from(rhs.0));
            let (mut q, mut r) = (&a / &b, &a % &b);
            if r < 0i8.into() {
                r += b.abs();
                q += 1
            }
            Some(Box::new(PairItem::new(
                MutezItem(q.try_into()?).into(), 
                MutezItem(r.try_into()?).into()
            ).into()))
        };
        Ok(OptionItem { outer_value, inner_type })
    }
}

impl Div<MutezItem> for MutezItem {
    type Output = Result<OptionItem>;

    fn div(self, rhs: MutezItem) -> Self::Output {
        let inner_type = pair(vec![nat(), mutez()]);
        let outer_value = if rhs.0 == 0u8.into() {
            None
        } else {
            let (a, b) = (IBig::from(self.0), IBig::from(rhs.0));
            let (mut q, mut r) = (&a / &b, &a % &b);
            if r < 0i8.into() {
                r += b.abs();
                q += 1
            }
            Some(Box::new(PairItem::new(
                NatItem(q.try_into()?).into(), 
                MutezItem(r.try_into()?).into()
            ).into()))
        };
        Ok(OptionItem { outer_value, inner_type })
    }
}

impl Shl<NatItem> for NatItem {
    type Output = Result<NatItem>;

    fn shl(self, rhs: NatItem) -> Self::Output {
        if rhs.0 >= 257u16.into() {
            return Err(Error::ShiftOverflow);
        }
        let shift: usize = rhs.0.try_into()?;
        Ok(NatItem(self.0 << shift))
    }
}

impl Shr<NatItem> for NatItem {
    type Output = Result<NatItem>;

    fn shr(self, rhs: NatItem) -> Self::Output {
        if rhs.0 >= 257u16.into() {
            return Err(Error::ShiftOverflow);
        }
        let shift: usize = rhs.0.try_into()?;
        Ok(NatItem(self.0 >> shift))
    }
}

impl Mul<NatItem> for NatItem {
    type Output = NatItem;

    fn mul(self, rhs: NatItem) -> Self::Output {
        NatItem(self.0 * rhs.0)
    }
}

impl Mul<IntItem> for IntItem {
    type Output = IntItem;

    fn mul(self, rhs: IntItem) -> Self::Output {
        IntItem(self.0 * rhs.0)
    }
}

impl Mul<NatItem> for MutezItem {
    type Output = Result<MutezItem>;

    fn mul(self, rhs: NatItem) -> Self::Output {
        let b: i64 = rhs.0.try_into()?;
        MutezItem::new(self.0 * b)
    }
}

impl Neg for IntItem {
    type Output = IntItem;

    fn neg(self) -> Self::Output {
        IntItem(-self.0)
    }
}

impl Neg for NatItem {
    type Output = IntItem;

    fn neg(self) -> Self::Output {
        IntItem(-IBig::from(self.0))
    }
}

impl Sub<NatItem> for NatItem {
    type Output = IntItem;

    fn sub(self, rhs: NatItem) -> Self::Output {
        IntItem(IBig::from(self.0) - IBig::from(rhs.0))
    }
}

impl Sub<IntItem> for NatItem {
    type Output = IntItem;

    fn sub(self, rhs: IntItem) -> Self::Output {
        IntItem(IBig::from(self.0) - rhs.0)
    }
}

impl Sub<NatItem> for IntItem {
    type Output = IntItem;

    fn sub(self, rhs: NatItem) -> Self::Output {
        IntItem(self.0 - IBig::from(rhs.0))
    }
}

impl Sub<IntItem> for IntItem {
    type Output = IntItem;

    fn sub(self, rhs: IntItem) -> Self::Output {
        IntItem(self.0 - rhs.0)
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

impl Sub<MutezItem> for MutezItem {
    type Output = Result<MutezItem>;

    fn sub(self, rhs: MutezItem) -> Self::Output {
        MutezItem::new(self.0 - rhs.0)
    }
}

impl BitOr<NatItem> for NatItem {
    type Output = NatItem;

    fn bitor(self, rhs: NatItem) -> Self::Output {
        NatItem(self.0 | rhs.0)
    }
}

impl BitXor<NatItem> for NatItem {
    type Output = NatItem;

    fn bitxor(self, rhs: NatItem) -> Self::Output {
        NatItem(self.0 ^ rhs.0)
    }
}

impl BitAnd<NatItem> for NatItem {
    type Output = NatItem;

    fn bitand(self, rhs: NatItem) -> Self::Output {
        NatItem(self.0 & rhs.0)
    }
}

impl BitAnd<NatItem> for IntItem {
    type Output = Result<NatItem>;

    fn bitand(self, rhs: NatItem) -> Self::Output {
        let a: i128 = self.0.try_into()?;  // FIXME: find generic solution
        Ok(NatItem(a & rhs.0))
    }
}

impl Not for IntItem {
    type Output = IntItem;

    fn not(self) -> Self::Output {
        IntItem(!self.0)
    }
}

impl Not for NatItem {
    type Output = IntItem;

    fn not(self) -> Self::Output {
        IntItem(!IBig::from(self.0))
    }
}