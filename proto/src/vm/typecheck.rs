use ibig::{IBig, UBig};
use chrono::{DateTime, NaiveDateTime, Utc};
use tezos_core::types::{
    encoded::{Address, PublicKey, ImplicitAddress, Signature, Encoded}
};
use tezos_michelson::michelson::{
    data::Data,
    data,
    types::{Type, ComparableType},
    types
};

use crate::{
    vm::stack::*,
    Result,
    error::Error
};

pub trait Typecheck {
    fn from_data(data: Data, ty: &Type) -> Result<StackItem>;
    fn type_check(&self, ty: &Type) -> Result<()>;
    fn into_data(self, ty: &Type) -> Result<Data>;
}

impl Typecheck for StackItem {
    fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        match ty {
            Type::Comparable(ComparableType::Unit(_)) => UnitItem::from_data(data, ty),
            _ => Err(Error::MichelsonTypeUnsupported { ty: ty.clone() })
        }
    }

    fn type_check(&self, ty: &Type) -> Result<()> {
        match self {
            StackItem::Unit(val) => val.type_check(ty),
            _ => Err(Error::MichelsonTypeUnsupported { ty: ty.clone() })
        }
    }

    fn into_data(self, ty: &Type) -> Result<Data> {
        match self {
            StackItem::Unit(val) => val.into_data(ty),
            _ => Err(Error::MichelsonTypeUnsupported { ty: ty.clone() })
        }
    }
}

macro_rules! err_type {
    ($ty: expr, $data: expr) => {
        Err(Error::MichelsonTypeError { ty: $ty.clone(), data: format!("{:#?}", $data) })
    };
}

macro_rules! type_check_comparable {
    ($cmp_ty: ident) => {
        fn type_check(&self, ty: &Type) -> Result<()> {
            match ty {
                Type::Comparable(ComparableType::$cmp_ty(_)) => Ok(()),
                _ => err_type!(ty, self)
            }
        }
    };
}

macro_rules! type_check_generic {
    ($cmp_ty: ident) => {
        fn type_check(&self, ty: &Type) -> Result<()> {
            match ty {
                Type::$cmp_ty(_) => Ok(()),
                _ => err_type!(ty, self)
            }
        }
    };
}

impl Typecheck for UnitItem {
    type_check_comparable!(Unit);

    fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        match data {
            Data::Unit(_) => Ok(StackItem::Unit(Self(()))),
            _ => err_type!(ty, data)
        }
    }

    fn into_data(self, ty: &Type) -> Result<Data> {
        self.type_check(ty)?;
        Ok(Data::Unit(data::unit()))
    }
}

impl Typecheck for BytesItem {
    type_check_comparable!(Bytes);

    fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        match data {
            Data::Bytes(val) => Ok(StackItem::Bytes(Self((&val).into()))),
            _ => err_type!(ty, data)
        }
    }

    fn into_data(self, ty: &Type) -> Result<Data> {
        self.type_check(ty)?;
        Ok(Data::Bytes(data::bytes(self.0)))
    }
}

impl Typecheck for StringItem {
    type_check_comparable!(String);

    fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        match data {
            Data::String(val) => Ok(StackItem::String(Self(val.into_string()))),
            _ => err_type!(ty, data)
        }
    }

    fn into_data(self, ty: &Type) -> Result<Data> {
        self.type_check(ty)?;
        Ok(Data::String(data::String::from_string(self.0)?))
    }
}

impl Typecheck for IntItem {
    type_check_comparable!(Int);

    fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        match data {
            Data::Int(val) => Ok(StackItem::Int(IBig::from_str_radix(val.to_str(), 10)?.into())),
            _ => err_type!(ty, data)
        }
    }

    fn into_data(self, ty: &Type) -> Result<Data> {
        self.type_check(ty)?;
        Ok(Data::Int(self.0.into()))
    }
}

impl Typecheck for NatItem {
    type_check_comparable!(Nat);

    fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        match data {
            // TODO: Data::Int ??
            Data::Nat(val) => Ok(StackItem::Nat(UBig::from_str_radix(val.to_str(), 10)?.into())),
            _ => err_type!(ty, data)
        }
    }

    fn into_data(self, ty: &Type) -> Result<Data> {
        self.type_check(ty)?;
        Ok(Data::Nat(self.0.into()))
    }
}

impl Typecheck for BoolItem {
    type_check_comparable!(Bool);

    fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        match data {
            Data::True(_) => return Ok(StackItem::Bool(true.into())),
            Data::False(_) => return Ok(StackItem::Bool(false.into())),
            _ => err_type!(ty, data)
        }
    }

    fn into_data(self, ty: &Type) -> Result<Data> {
        self.type_check(ty)?;
        match self.0 {
            true => Ok(Data::True(data::True)),
            false => Ok(Data::False(data::False))
        }
    }
}

pub fn int_to_i64(value: data::Int, ty: &Type) -> Result<i64> {
    let int: i64 = value.to_integer()?;
    if int >= 0 { Ok(int) } else { err_type!(ty, value) }
}

impl Typecheck for TimestampItem {
    type_check_comparable!(Timestamp);

    fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        let timestamp = match data {
            Data::String(val) => DateTime::parse_from_rfc3339(val.to_str())?.timestamp(),
            Data::Int(val) => int_to_i64(val, ty)?,
            _ => return err_type!(ty, data)
        };
        Ok(StackItem::Timestamp(timestamp.into()))
    }

    fn into_data(self, ty: &Type) -> Result<Data> {
        self.type_check(ty)?;
        let dt = match NaiveDateTime::from_timestamp_opt(self.0, 0) {
            Some(dt) => DateTime::<Utc>::from_utc(dt, Utc),
            None => return err_type!(ty, self)
        };
        Ok(Data::String(data::String::from_string(dt.to_rfc3339())?))
    }
}

impl Typecheck for MutezItem {
    type_check_comparable!(Mutez);

    fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        match data {
            Data::Int(val) => Ok(StackItem::Mutez(int_to_i64(val, ty)?.into())),
            _ => err_type!(ty, data)
        }
    }

    fn into_data(self, ty: &Type) -> Result<Data> {
        self.type_check(ty)?;
        Ok(Data::Int(self.0.into()))
    }
}

macro_rules! impl_for_encoded {
    ($item_ty: ident, $impl_ty: ty, $cmp_ty: ident) => {
        impl Typecheck for $item_ty {
            type_check_comparable!($cmp_ty);

            fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
                match data {
                    Data::String(val) => Ok($item_ty(<$impl_ty>::new(val.into_string())?).into()),
                    _ => err_type!(ty, data)
                }
            }
                
            fn into_data(self, ty: &Type) -> Result<Data> {
                self.type_check(ty)?;
                Ok(Data::String(data::String::from_string(self.0.into_string())?))
            }
        }
    };
}

impl_for_encoded!(AddressItem, Address, Address);
impl_for_encoded!(KeyItem, PublicKey, Key);
impl_for_encoded!(KeyHashItem, ImplicitAddress, KeyHash);
impl_for_encoded!(SignatureItem, Signature, Signature);

impl Typecheck for OptionItem {
    type_check_comparable!(Option);

    fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        if let Type::Option(option_ty) = ty {
            return match data {
                Data::None(_) => Ok(StackItem::Option(Self(None))),
                Data::Some(val) => {
                    let inner = StackItem::from_data(*val.value, &option_ty.r#type)?;
                    Ok(StackItem::Option(Self(Some(Box::new(inner)))))
                },
                _ => err_type!(ty, data)
            }
        }
        err_type!(ty, "OptionItem")  
    }

    fn into_data(self, ty: &Type) -> Result<Data> {
        if let Type::Option(option_ty) = ty {
            return match self.0 {
                None => Ok(Data::None(data::none())),
                Some(val) => {
                    let inner = (*val).into_data(&option_ty.r#type)?;
                    Ok(Data::Some(data::some(inner)))
                }
            }
        }
        err_type!(ty, self)
    }
}

impl Typecheck for OrItem {
    type_check_comparable!(Or);

    fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        if let Type::Or(or_ty) = ty {
            return match data {
                Data::Left(left) => {
                    let inner = StackItem::from_data(*left.value, &or_ty.lhs)?;
                    Ok(StackItem::Or(Self::Left(Box::new(inner))))
                },
                Data::Right(right) => {
                    let inner = StackItem::from_data(*right.value, &or_ty.rhs)?;
                    Ok(StackItem::Or(Self::Right(Box::new(inner))))
                },
                _ => err_type!(ty, data)
            }
        }
        err_type!(ty, "OrItem")
    }
    
    fn into_data(self, ty: &Type) -> Result<Data> {
        if let Type::Or(or_ty) = ty {
            return match self {
                Self::Left(left) => {
                    let inner = left.into_data(&or_ty.lhs)?;
                    Ok(Data::Left(data::left(inner)))
                },
                Self::Right(right) => {
                    let inner = right.into_data(&or_ty.rhs)?;
                    Ok(Data::Right(data::right(inner)))
                }
            }
        }
        err_type!(ty, self)
    }
}

impl Typecheck for PairItem {
    type_check_comparable!(Pair);

    fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        if let Type::Pair(pair_ty) = ty {
            assert_eq!(2, pair_ty.types.len());
            return match data {
                Data::Pair(pair) => {
                    assert_eq!(2, pair.values.len());
                    let car = StackItem::from_data(pair.values[0].clone(), &pair_ty.types[0])?;
                    let cdr = StackItem::from_data(pair.values[1].clone(), &pair_ty.types[1])?;
                    Ok(StackItem::Pair(Self((Box::new(car), Box::new(cdr)))))
                },
                _ => err_type!(ty, data)
            }
        }
        err_type!(ty, "PairItem")
    }
    
    fn into_data(self, ty: &Type) -> Result<Data> {
        if let Type::Pair(pair_ty) = ty {
            assert_eq!(2, pair_ty.types.len());
            let car = self.0.0.into_data(&pair_ty.types[0])?;
            let cdr = self.0.1.into_data(&pair_ty.types[1])?;
            return Ok(Data::Pair(data::pair(vec![car, cdr])))
        }
        err_type!(ty, self)
    }
}

impl Typecheck for ListItem {
    type_check_generic!(List);

    fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        if let Type::List(list_ty) = ty {
            return match data {
                Data::Sequence(seq) => {
                    let values = seq.into_values();
                    let mut items: Vec<StackItem> = Vec::with_capacity(values.len());
                    for value in values {
                        items.push(StackItem::from_data(value, &list_ty.r#type)?);
                    }
                    Ok(StackItem::List(Self(items)))
                },
                _ => err_type!(ty, data)
            }
        }
        err_type!(ty, "ListItem")
    }

    fn into_data(self, ty: &Type) -> Result<Data> {
        if let Type::List(list_ty) = ty {
            let mut values: Vec<Data> = Vec::with_capacity(self.0.len());
            for item in self.0 {
                values.push(item.into_data(&list_ty.r#type)?);
            }
            return Ok(Data::Sequence(data::sequence(values)))
        }
        err_type!(ty, self)
    }
}

impl Typecheck for SetItem {
    type_check_generic!(Set);

    fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        if let Type::Set(set_ty) = ty {
            let val_ty = Type::Comparable(set_ty.r#type.clone());
            return match data {
                Data::Sequence(seq) => {
                    let values = seq.into_values();
                    let mut items: Vec<StackItem> = Vec::with_capacity(values.len());
                    for value in values {
                        items.push(StackItem::from_data(value, &val_ty)?);
                    }
                    Ok(StackItem::Set(Self(items)))
                },
                _ => err_type!(ty, data)
            }
        }
        err_type!(ty, "SetItem")
    }

    fn into_data(self, ty: &Type) -> Result<Data> {
        if let Type::Set(set_ty) = ty {
            let val_ty = Type::Comparable(set_ty.r#type.clone());
            let mut values: Vec<Data> = Vec::with_capacity(self.0.len());
            for item in self.0 {
                values.push(item.into_data(&val_ty)?);
            }
            return Ok(Data::Sequence(data::sequence(values)))
        }
        err_type!(ty, self)
    }
}

fn seq_to_map(sequence: data::Sequence, key_ty: &Type, val_ty: &Type, ty: &Type) -> Result<MapItem> {
    let elements = sequence.into_values();
    let mut items: Vec<(StackItem, StackItem)> = Vec::with_capacity(elements.len());
    for element in elements {
        if let Data::Elt(elt) = element {
            let key = StackItem::from_data(*elt.key, &key_ty)?;
            let val = StackItem::from_data(*elt.value, &val_ty)?;
            items.push((key, val));
        } else {
            return err_type!(ty, element)
        }
    }
    return Ok(MapItem(items))
}

impl Typecheck for MapItem {
    type_check_generic!(Map);

    fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        if let Type::Map(map_ty) = ty {
            return match data {
                Data::Sequence(seq) => {
                    let map_item = seq_to_map(seq, &map_ty.key_type, &map_ty.value_type, ty)?;
                    Ok(StackItem::Map(map_item))
                },
                _ => err_type!(ty, data)
            } 
        }
        err_type!(ty, "MapItem")
    }

    fn into_data(self, ty: &Type) -> Result<Data> {
        if let Type::Map(map_ty) = ty {
            let mut elements: Vec<Data> = Vec::with_capacity(self.0.len());
            for (key_item, val_item) in self.0 {
                let key = key_item.into_data(&map_ty.key_type)?;
                let value = val_item.into_data(&map_ty.value_type)?;
                elements.push(Data::Elt(data::elt(key, value)));
            }
            return Ok(Data::Sequence(data::sequence(elements)))
        }
        err_type!(ty, self)
    }
}

impl Typecheck for BigMapItem {
    type_check_generic!(BigMap);

    fn from_data(data: Data, ty: &Type) -> Result<StackItem> {
        if let Type::BigMap(big_map_ty) = ty {
            return match data {
                Data::Int(ptr) => Ok(StackItem::BigMap(Self::Ptr(ptr.to_integer()?))),
                Data::Sequence(seq) => {
                    let inner = seq_to_map(seq, &big_map_ty.key_type, &big_map_ty.value_type, ty)?;
                    Ok(StackItem::BigMap(Self::Map(inner)))
                },
                _ => err_type!(ty, data)
            }
        }
        err_type!(ty, "BigMapItem")
    }

    fn into_data(self, ty: &Type) -> Result<Data> {
        if let Type::BigMap(_) = ty {
            return match self {
                Self::Ptr(ptr) => Ok(Data::Int(data::int(ptr))),
                Self::Map(_) => err_type!(ty, self)
            }
        }
        err_type!(ty, self)
    }
}