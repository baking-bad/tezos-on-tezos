use ibig::{IBig, UBig};
use std::fmt::Display;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Shl, Shr, Sub};
use tezos_michelson::michelson::{
    data::Data,
    types::{nat, pair, ComparableType, Type},
};

use crate::{
    comparable_type_cast, err_mismatch,
    formatter::Formatter,
    types::{IntItem, NatItem, OptionItem, PairItem, StackItem},
    Error, Result,
};

impl NatItem {
    pub fn from_data(data: Data) -> Result<StackItem> {
        let val = match data {
            Data::Int(val) => UBig::from_str_radix(val.to_str(), 10)?,
            Data::Nat(val) => UBig::from_str_radix(val.to_str(), 10)?,
            _ => return err_mismatch!("Int or Nat", data.format()),
        };
        Ok(StackItem::Nat(val.into()))
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        comparable_type_cast!(ty, Nat);
        let int: IBig = self.0.into();
        Ok(Data::Int(int.into()))
    }

    pub fn int(self) -> IntItem {
        IntItem(IBig::from(self.0))
    }
}

impl Display for NatItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}n", self.0.to_string()))
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

impl Mul<NatItem> for NatItem {
    type Output = NatItem;

    fn mul(self, rhs: NatItem) -> Self::Output {
        NatItem(self.0 * rhs.0)
    }
}

impl Div<NatItem> for NatItem {
    type Output = OptionItem;

    fn div(self, rhs: NatItem) -> Self::Output {
        if rhs.0 == 0u8.into() {
            OptionItem::None(pair(vec![nat(), nat()]))
        } else {
            let (q, r) = (&self.0 / &rhs.0, &self.0 % &rhs.0);
            let res = PairItem::new(NatItem(q).into(), NatItem(r).into());
            OptionItem::some(res.into())
        }
    }
}

impl Shl<NatItem> for NatItem {
    type Output = Result<NatItem>;

    fn shl(self, rhs: NatItem) -> Self::Output {
        if rhs.0 >= 257u16.into() {
            return Err(Error::GeneralOverflow.into());
        }
        let shift: usize = rhs.0.try_into()?;
        Ok(NatItem(self.0 << shift))
    }
}

impl Shr<NatItem> for NatItem {
    type Output = Result<NatItem>;

    fn shr(self, rhs: NatItem) -> Self::Output {
        if rhs.0 >= 257u16.into() {
            return Err(Error::GeneralOverflow.into());
        }
        let shift: usize = rhs.0.try_into()?;
        Ok(NatItem(self.0 >> shift))
    }
}

impl Neg for NatItem {
    type Output = IntItem;

    fn neg(self) -> Self::Output {
        IntItem(-IBig::from(self.0))
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

impl Not for NatItem {
    type Output = IntItem;

    fn not(self) -> Self::Output {
        IntItem(!IBig::from(self.0))
    }
}

impl TryInto<usize> for NatItem {
    type Error = Error;

    fn try_into(self) -> Result<usize> {
        let res: usize = self.0.try_into()?;
        Ok(res)
    }
}

impl TryFrom<i32> for NatItem {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self> {
        Ok(Self(UBig::try_from(value)?))
    }
}
