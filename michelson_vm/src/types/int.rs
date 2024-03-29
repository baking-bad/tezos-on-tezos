// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use ibig::ops::{Abs, UnsignedAbs};
use ibig::{IBig, UBig};
use std::fmt::Display;
use std::ops::{Add, BitAnd, Div, Mul, Neg, Not, Sub};
use tezos_michelson::michelson::{
    data::Data,
    types::{int, nat, pair, ComparableType, Type},
};

use crate::{
    comparable_type_cast, err_mismatch,
    formatter::Formatter,
    types::{IntItem, NatItem, OptionItem, PairItem, StackItem},
    Result,
};

impl IntItem {
    pub fn from_data(data: Data) -> Result<StackItem> {
        match data {
            Data::Int(val) => Ok(StackItem::Int(IntItem(val.into()))),
            _ => err_mismatch!("Int", data.format()),
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        comparable_type_cast!(ty, Int);
        Ok(Data::Int(self.0.into()))
    }

    pub fn abs(self) -> Result<NatItem> {
        Ok(NatItem(self.0.unsigned_abs()))
    }

    pub fn nat(self) -> Result<OptionItem> {
        if self.0 >= 0.into() {
            let nat = NatItem(self.0.try_into()?);
            Ok(OptionItem::Some(Box::new(StackItem::Nat(nat))))
        } else {
            Ok(OptionItem::None(nat()))
        }
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
        if rhs.0 == 0i8.into() {
            OptionItem::None(pair(vec![int(), nat()]))
        } else {
            let (mut q, mut r) = (&self.0 / &rhs.0, &self.0 % &rhs.0);
            if r < 0i8.into() {
                r += rhs.0.abs();
                q += 1
            }
            OptionItem::some(
                PairItem::new(
                    IntItem(q).into(),
                    NatItem(UBig::try_from(r).expect("positive remainder")).into(),
                )
                .into(),
            )
        }
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
        let a: i128 = self.0.try_into()?; // FIXME: find generic solution
        Ok(NatItem(a & rhs.0))
    }
}
