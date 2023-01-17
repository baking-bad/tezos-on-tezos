use tezos_michelson::micheline::Micheline;
use tezos_michelson::michelson::{
    data::Data,
    types,
    types::{ComparableType, Type},
    Michelson,
};

use crate::{err_mismatch, err_unsupported, formatter::Formatter, types::*, Result};

#[macro_export]
macro_rules! type_cast {
    ($typ: expr, $var: ident) => {
        match $typ {
            Type::$var(var) => var,
            _ => return err_mismatch!($typ.format(), stringify!($var)),
        }
    };
}

#[macro_export]
macro_rules! comparable_type_cast {
    ($typ: expr, $var: ident) => {
        match $typ {
            Type::Comparable(ComparableType::$var(var)) => var,
            _ => return err_mismatch!($typ.format(), stringify!($var)),
        }
    };
}

pub fn type_comparable(ty: &Type) -> bool {
    match ty {
        Type::Comparable(_) => true,
        Type::Option(option_ty) => type_comparable(&option_ty.r#type),
        Type::Or(or_ty) => type_comparable(&or_ty.lhs) && type_comparable(&or_ty.rhs),
        Type::Pair(pair_ty) => pair_ty.types.iter().all(type_comparable),
        _ => false,
    }
}

pub fn check_pair_len(len: usize) -> Result<()> {
    match len {
        2 => Ok(()),
        i => err_mismatch!("2 args", i),
    }
}

pub fn check_type_comparable(ty: &Type) -> Result<()> {
    match type_comparable(ty) {
        true => Ok(()),
        false => err_mismatch!("ComparableType", ty.format()),
    }
}

pub fn comparable_types_equal(lhs: &ComparableType, rhs: &ComparableType) -> Result<bool> {
    use ::core::mem::discriminant;
    let ltag = discriminant(lhs);
    let rtag = discriminant(rhs);
    if ltag != rtag {
        return Ok(false);
    }
    match lhs {
        ComparableType::Unit(_) => Ok(true),
        ComparableType::Bool(_) => Ok(true),
        ComparableType::String(_) => Ok(true),
        ComparableType::Bytes(_) => Ok(true),
        ComparableType::Int(_) => Ok(true),
        ComparableType::Nat(_) => Ok(true),
        ComparableType::Mutez(_) => Ok(true),
        ComparableType::Timestamp(_) => Ok(true),
        ComparableType::Address(_) => Ok(true),
        ComparableType::Key(_) => Ok(true),
        ComparableType::KeyHash(_) => Ok(true),
        ComparableType::Signature(_) => Ok(true),
        _ => err_unsupported!(lhs.format()),
    }
}

pub fn types_equal(lhs: &Type, rhs: &Type) -> Result<bool> {
    use ::core::mem::discriminant;
    let ltag = discriminant(lhs);
    let rtag = discriminant(rhs);
    if ltag != rtag {
        return Ok(false);
    }
    match (lhs, rhs) {
        (Type::Comparable(lty), Type::Comparable(rty)) => comparable_types_equal(lty, rty),
        (Type::Option(lty), Type::Option(rty)) => types_equal(&lty.r#type, &rty.r#type),
        (Type::Or(lty), Type::Or(rty)) => {
            types_equal(&lty.lhs, &rty.lhs)?;
            types_equal(&lty.rhs, &rty.rhs)
        }
        (Type::Pair(lty), Type::Pair(rty)) => {
            check_pair_len(lty.types.len())?;
            check_pair_len(rty.types.len())?;
            types_equal(&lty.types[0], &rty.types[0])?;
            types_equal(&lty.types[1], &rty.types[1])
        }
        (Type::List(lty), Type::List(rty)) => types_equal(&lty.r#type, &rty.r#type),
        (Type::Set(lty), Type::Set(rty)) => types_equal(&lty.r#type, &rty.r#type),
        (Type::Map(lty), Type::Map(rty)) => {
            types_equal(&lty.key_type, &rty.key_type)?;
            types_equal(&lty.value_type, &rty.value_type)
        }
        (Type::BigMap(lty), Type::BigMap(rty)) => {
            types_equal(&lty.key_type, &rty.key_type)?;
            types_equal(&lty.value_type, &rty.value_type)
        }
        (Type::Lambda(lty), Type::Lambda(rty)) => {
            types_equal(&lty.parameter_type, &rty.parameter_type)?;
            types_equal(&lty.return_type, &rty.return_type)
        }
        (Type::Contract(lty), Type::Contract(rty)) => types_equal(&lty.r#type, &rty.r#type),
        (Type::Parameter(lty), Type::Parameter(rty)) => types_equal(&lty.r#type, &rty.r#type),
        (Type::Storage(lty), Type::Storage(rty)) => types_equal(&lty.r#type, &rty.r#type),
        _ => err_unsupported!(lhs.format()),
    }
}

pub fn check_types_equal(lhs: &Type, rhs: &Type) -> Result<()> {
    match types_equal(lhs, rhs) {
        Ok(true) => Ok(()),
        Ok(false) => err_mismatch!(lhs.format(), rhs.format()),
        Err(err) => Err(err),
    }
}

impl StackItem {
    pub fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        match ty {
            Type::Comparable(cmp) => match cmp {
                ComparableType::Unit(_) => UnitItem::from_data(data),
                ComparableType::Bool(_) => BoolItem::from_data(data),
                ComparableType::String(_) => StringItem::from_data(data),
                ComparableType::Bytes(_) => BytesItem::from_data(data),
                ComparableType::Int(_) => IntItem::from_data(data),
                ComparableType::Nat(_) => NatItem::from_data(data),
                ComparableType::Mutez(_) => MutezItem::from_data(data),
                ComparableType::Timestamp(_) => TimestampItem::from_data(data),
                ComparableType::Address(_) => AddressItem::from_data(data),
                ComparableType::Key(_) => KeyItem::from_data(data),
                ComparableType::KeyHash(_) => KeyHashItem::from_data(data),
                ComparableType::Signature(_) => SignatureItem::from_data(data),
                ComparableType::ChainId(_) => ChainIdItem::from_data(data),
                _ => err_unsupported!(ty.format()),
            },
            Type::Option(option_ty) => OptionItem::from_data(data, &option_ty.r#type),
            Type::Or(or_ty) => OrItem::from_data(data, &or_ty.lhs, &or_ty.rhs),
            Type::Pair(pair_ty) => {
                check_pair_len(pair_ty.types.len())?;
                PairItem::from_data(data, &pair_ty.types[0], &pair_ty.types[1])
            }
            Type::List(list_ty) => ListItem::from_data(data, &list_ty.r#type),
            Type::Set(set_ty) => SetItem::from_data(data, &set_ty.r#type),
            Type::Map(map_ty) => MapItem::from_data(data, &map_ty.key_type, &map_ty.value_type),
            Type::BigMap(map_ty) => {
                BigMapItem::from_data(data, &map_ty.key_type, &map_ty.value_type)
            }
            Type::Lambda(lambda_ty) => {
                LambdaItem::from_data(data, &lambda_ty.parameter_type, &lambda_ty.return_type)
            }
            Type::Contract(contract_ty) => ContractItem::from_data(data, &contract_ty.r#type),
            Type::Parameter(param_ty) => StackItem::from_data(data, &param_ty.r#type),
            Type::Storage(storage_ty) => StackItem::from_data(data, &storage_ty.r#type),
            _ => err_unsupported!(ty.format()),
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
            StackItem::ChainId(item) => item.into_data(ty),
            StackItem::Option(item) => item.into_data(ty),
            StackItem::Or(item) => item.into_data(ty),
            StackItem::Pair(item) => item.into_data(ty),
            StackItem::List(item) => item.into_data(ty),
            StackItem::Set(item) => item.into_data(ty),
            StackItem::Map(item) => item.into_data(ty),
            StackItem::BigMap(item) => item.into_data(),
            StackItem::Lambda(item) => item.into_data(ty),
            StackItem::Contract(item) => item.into_data(ty),
            _ => err_unsupported!(ty.format()),
        }
    }

    pub fn get_type(&self) -> Result<Type> {
        match self {
            StackItem::Unit(_) => Ok(types::unit()),
            StackItem::Bytes(_) => Ok(types::bytes()),
            StackItem::String(_) => Ok(types::string()),
            StackItem::Int(_) => Ok(types::int()),
            StackItem::Nat(_) => Ok(types::nat()),
            StackItem::Bool(_) => Ok(types::bool()),
            StackItem::Timestamp(_) => Ok(types::timestamp()),
            StackItem::Mutez(_) => Ok(types::mutez()),
            StackItem::Address(_) => Ok(types::address()),
            StackItem::Key(_) => Ok(types::key()),
            StackItem::KeyHash(_) => Ok(types::key_hash()),
            StackItem::Signature(_) => Ok(types::signature()),
            StackItem::ChainId(_) => Ok(types::chain_id()),
            StackItem::Option(item) => item.get_type(),
            StackItem::Or(item) => item.get_type(),
            StackItem::Pair(item) => item.get_type(),
            StackItem::List(item) => Ok(item.get_type()),
            StackItem::Set(item) => item.get_type(),
            StackItem::Map(item) => item.get_type(),
            StackItem::BigMap(item) => item.get_type(),
            StackItem::Operation(_) => Ok(types::operation()),
            StackItem::Lambda(item) => item.get_type(),
            StackItem::Contract(item) => Ok(item.get_type()),
        }
    }

    pub fn type_check(&self, ty: &Type) -> Result<()> {
        let item_ty = self.get_type()?;
        match types_equal(ty, &item_ty)? {
            true => Ok(()),
            false => err_mismatch!(ty.format(), item_ty.format()),
        }
    }

    pub fn from_micheline(expr: Micheline, ty: &Type) -> Result<Self> {
        Self::from_data(expr.try_into()?, ty)
    }

    pub fn from_bytes(data: Vec<u8>, ty: &Type) -> Result<Self> {
        let src = Michelson::unpack(data.as_slice(), Some(ty))?;
        Self::from_data(src.try_into()?, ty)
    }

    pub fn into_micheline(self, ty: &Type) -> Result<Micheline> {
        Ok(self.into_data(ty)?.into())
    }
}
