use std::fmt::Display;
use std::ops::{Add, Div, Shl, Shr, Mul, Neg, Sub, BitOr, BitXor, BitAnd, Not};
use ibig::{IBig, UBig};
use tezos_michelson::michelson::{
    data::Data,
    types::{Type, ComparableType, nat, pair}
};

use crate::{
    Result, Error,
    error::InterpreterError,
    vm::types::{IntItem, NatItem, StackItem, OptionItem, PairItem},
    err_type,
    type_check_fn_comparable,
};

impl NatItem {
    type_check_fn_comparable!(Nat);

    pub fn from_data(data: Data) -> Result<StackItem> {
        match data {
            Data::Int(val) => Ok(StackItem::Nat(UBig::from_str_radix(val.to_str(), 10)?.into())),
            _ => err_type!("Data::Int", data)
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


impl Shl<NatItem> for NatItem {
    type Output = Result<NatItem>;

    fn shl(self, rhs: NatItem) -> Self::Output {
        if rhs.0 >= 257u16.into() {
            return Err(InterpreterError::GeneralOverflow.into());
        }
        let shift: usize = rhs.0.try_into()?;
        Ok(NatItem(self.0 << shift))
    }
}

impl Shr<NatItem> for NatItem {
    type Output = Result<NatItem>;

    fn shr(self, rhs: NatItem) -> Self::Output {
        if rhs.0 >= 257u16.into() {
            return Err(InterpreterError::GeneralOverflow.into());
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