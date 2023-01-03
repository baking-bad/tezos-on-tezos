use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};
use ibig::{IBig, UBig};
use ibig::ops::Abs;
use tezos_michelson::michelson::{
    data::Data,
    types::{Type, ComparableType, nat, mutez, pair}
};

use crate::{
    Result,
    Error,
    error::InterpreterError,
    vm::types::{NatItem, MutezItem, StackItem, OptionItem, PairItem},
    err_type,
    type_check_fn_comparable,
};

impl MutezItem {
    type_check_fn_comparable!(Mutez);

    pub fn new(value: i64) -> Result<Self> {
        if value < 0 {
            return Err(InterpreterError::MutezUnderflow.into())
        }
        Ok(Self(value)) // TODO: check overflow
    }

    pub fn from_data(data: Data) -> Result<StackItem> {
        match data {
            Data::Int(val) => {
                match val.to_integer::<i64>() {
                    Ok(val) => Ok(Self::new(val)?.into()),
                    Err(_) => Err(InterpreterError::MutezOverflow.into())
                }                
            },
            _ => err_type!("Data::Int", data)
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        self.type_check(ty)?;
        Ok(Data::Int(self.0.into()))
    }
}

impl Display for MutezItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}utz", self.0))
    }
}

impl TryFrom<IBig> for MutezItem {
    type Error = Error;

    fn try_from(value: IBig) -> Result<Self> {
        match value.try_into() {
            Ok(val) => MutezItem::new(val),
            Err(_) => Err(InterpreterError::MutezOverflow.into())
        }
    }
}

impl TryFrom<UBig> for MutezItem {
    type Error = Error;

    fn try_from(value: UBig) -> Result<Self> {
        match value.try_into() {
            Ok(val) => MutezItem::new(val),
            Err(_) => Err(InterpreterError::MutezOverflow.into())
        }
    }
}

impl Add<MutezItem> for MutezItem {
    type Output = Result<MutezItem>;

    fn add(self, rhs: MutezItem) -> Self::Output {
        match self.0.checked_add(rhs.0) {
            Some(res) => MutezItem::new(res),
            None => Err(InterpreterError::MutezOverflow.into())
        } 
    }
}

impl Sub<MutezItem> for MutezItem {
    type Output = Result<MutezItem>;

    fn sub(self, rhs: MutezItem) -> Self::Output {
        MutezItem::new(self.0 - rhs.0)
    }
}

impl Mul<NatItem> for MutezItem {
    type Output = Result<MutezItem>;

    fn mul(self, rhs: NatItem) -> Self::Output {
        (rhs.0 * self.0).try_into()
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
                MutezItem::try_from(q)?.into(), 
                MutezItem::try_from(r)?.into()
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
            let (a, b) = (self.0, rhs.0);
            let (mut q, mut r) = (a / b, a % b);
            if r < 0i8.into() {
                r += b.abs();
                q += 1
            }
            Some(Box::new(PairItem::new(
                NatItem(q.try_into()?).into(), 
                MutezItem::new(r)?.into()
            ).into()))
        };
        Ok(OptionItem { outer_value, inner_type })
    }
}
