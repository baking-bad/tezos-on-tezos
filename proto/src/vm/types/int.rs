use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Sub, BitAnd, Not};
use ibig::{IBig, UBig};
use ibig::ops::{Abs, UnsignedAbs};
use tezos_michelson::michelson::{
    data::Data,
    types::{Type, ComparableType, int, nat, pair}
};

use crate::{
    Result,
    vm::types::{IntItem, NatItem, StackItem, OptionItem, PairItem},
    err_type,
    type_check_fn_comparable,
};

impl IntItem {
    type_check_fn_comparable!(Int);

    pub fn from_data(data: Data) -> Result<StackItem> {
        match data {
            Data::Int(val) => Ok(StackItem::Int(IBig::from_str_radix(val.to_str(), 10)?.into())),
            _ => err_type!("Data::Int", data)
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
        let outer_value = if self.0 >= 0.into() {
            let nat = NatItem(self.0.try_into()?);
            Some(Box::new(StackItem::Nat(nat)))
        } else {
            None
        };
        Ok(OptionItem { outer_value, inner_type: nat() })
    }
}

impl From<i64> for IntItem {
    fn from(value: i64) -> Self {
        Self(value.into())
    }
}

impl Display for IntItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}

impl Add<IntItem> for IntItem {
    type Output = IntItem;

    fn add(self, rhs: IntItem) -> Self::Output {
        IntItem(self.0 + rhs.0)
    }
}

impl Sub<IntItem> for IntItem {
    type Output = IntItem;

    fn sub(self, rhs: IntItem) -> Self::Output {
        IntItem(self.0 - rhs.0)
    }
}

impl Sub<NatItem> for IntItem {
    type Output = IntItem;

    fn sub(self, rhs: NatItem) -> Self::Output {
        IntItem(self.0 - IBig::from(rhs.0))
    }
}

impl Mul<IntItem> for IntItem {
    type Output = IntItem;

    fn mul(self, rhs: IntItem) -> Self::Output {
        IntItem(self.0 * rhs.0)
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

impl Neg for IntItem {
    type Output = IntItem;

    fn neg(self) -> Self::Output {
        IntItem(-self.0)
    }
}

impl Not for IntItem {
    type Output = IntItem;

    fn not(self) -> Self::Output {
        IntItem(!self.0)
    }
}

impl BitAnd<NatItem> for IntItem {
    type Output = Result<NatItem>;

    fn bitand(self, rhs: NatItem) -> Self::Output {
        let a: i128 = self.0.try_into()?;  // FIXME: find generic solution
        Ok(NatItem(a & rhs.0))
    }
}