use tezos_michelson::michelson::{
    data::Data,
    types::{Type, ComparableType}
};
use tezos_michelson::micheline::Micheline;

use crate::{
    vm::types::*,
    Result,
    Error
};

#[macro_export]
macro_rules! err_type {
    ($expected: expr, $found: expr) => {
        Err(Error::MichelsonTypeError {
            expected: format!("{:#?}", $expected),
            found: format!("{:#?}", $found)
        })
    };
}

#[macro_export]
macro_rules! assert_types_equal {
    ($ty_exp: expr, $ty_act: expr) => {
        if $ty_exp != $ty_act {
            return err_type!($ty_exp, $ty_act);
        }
    };
}

#[macro_export]
macro_rules! type_check_comparable {
    ($cmp_ty: ident) => {
        pub fn type_check(&self, ty: &Type) -> Result<()> {
            match ty {
                Type::Comparable(ComparableType::$cmp_ty(_)) => Ok(()),
                _ => err_type!(ty, self)
            }
        }
    };
}

#[macro_export]
macro_rules! comparable_ref {
    ($arg: expr) => {
        &Type::Comparable($arg.clone())
    };
}

impl StackItem {
    pub fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        match ty {
            Type::Comparable(cmp) => match cmp {
                ComparableType::Unit(_) => UnitItem::from_data(data, ty),
                ComparableType::Bool(_) => BoolItem::from_data(data, ty),
                ComparableType::String(_) => StringItem::from_data(data, ty),
                ComparableType::Bytes(_) => BytesItem::from_data(data, ty),
                ComparableType::Int(_) => IntItem::from_data(data, ty),
                ComparableType::Nat(_) => NatItem::from_data(data, ty),
                ComparableType::Mutez(_) => MutezItem::from_data(data, ty),
                ComparableType::Timestamp(_) => TimestampItem::from_data(data, ty),
                ComparableType::Address(_) => AddressItem::from_data(data, ty),
                ComparableType::Key(_) => KeyItem::from_data(data, ty),
                ComparableType::KeyHash(_) => KeyHashItem::from_data(data, ty),
                ComparableType::Signature(_) => SignatureItem::from_data(data, ty),
                ComparableType::Option(option_ty) => {
                    OptionItem::from_data(data, ty, comparable_ref!(*option_ty.r#type))
                },
                ComparableType::Or(or_ty) => {
                    OrItem::from_data(data, ty, comparable_ref!(*or_ty.lhs), comparable_ref!(*or_ty.rhs))
                },
                ComparableType::Pair(pair_ty) => {
                    if pair_ty.types.len() != 2 {
                        Err(Error::UnexpectedPairArity)
                    } else {
                        PairItem::from_data(data, ty, comparable_ref!(pair_ty.types[0]), comparable_ref!(pair_ty.types[1]))
                    }
                },
                _ => Err(Error::MichelsonTypeUnsupported { ty: ty.clone() })
            },
            Type::Option(option_ty) => OptionItem::from_data(data, ty, &option_ty.r#type),
            Type::Or(or_ty) => OrItem::from_data(data, ty, &or_ty.lhs, &or_ty.rhs),
            Type::Pair(pair_ty) => {
                if pair_ty.types.len() != 2 {
                    Err(Error::UnexpectedPairArity)
                } else {
                    PairItem::from_data(data, ty, &pair_ty.types[0], &pair_ty.types[1])
                }
            },
            Type::List(list_ty) => ListItem::from_data(data, ty, &list_ty.r#type),
            Type::Set(set_ty) => SetItem::from_data(data, ty, comparable_ref!(set_ty.r#type)),
            Type::Map(map_ty) => MapItem::from_data(data, ty, &map_ty.key_type, &map_ty.value_type),
            Type::BigMap(map_ty) => BigMapItem::from_data(data, ty, &map_ty.key_type, &map_ty.value_type),
            Type::Parameter(param_ty) => StackItem::from_data(data, &param_ty.r#type),
            Type::Storage(storage_ty) => StackItem::from_data(data, &storage_ty.r#type),
            _ => Err(Error::MichelsonTypeUnsupported { ty: ty.clone() })
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        match self {
            StackItem::Unit(item) => item.into_data(ty),
            StackItem::Bytes(item) => item.into_data(ty),
            StackItem::String(item) => item.into_data(ty),
            StackItem::Int(item) => item.into_data(ty),
            StackItem::Nat(item) => item.into_data(ty),
            StackItem::Bool(item) => item.into_data(ty),
            StackItem::Timestamp(item) => item.into_data(ty),
            StackItem::Mutez(item) => item.into_data(ty),
            StackItem::Address(item) => item.into_data(ty),
            StackItem::Key(item) => item.into_data(ty),
            StackItem::KeyHash(item) => item.into_data(ty),
            StackItem::Signature(item) => item.into_data(ty),
            StackItem::Option(item) => item.into_data(ty),
            StackItem::Or(item) => item.into_data(ty),
            StackItem::Pair(item) => item.into_data(ty),
            StackItem::List(item) => item.into_data(ty),
            StackItem::Set(item) => item.into_data(ty),
            StackItem::Map(item) => item.into_data(ty),
            StackItem::BigMap(item) => item.into_data(ty),
            _ => Err(Error::MichelsonTypeUnsupported { ty: ty.clone() })
        }
    }

    pub fn from_micheline(expr: Micheline, ty: &Type) -> Result<Self> {
        Self::from_data(expr.try_into()?, ty)
    }

    pub fn into_micheline(self, ty: &Type) -> Result<Micheline> {
        Ok(self.into_data(ty)?.into())
    }
}
