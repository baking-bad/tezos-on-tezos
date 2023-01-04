use std::fmt::Display;
use tezos_michelson::michelson::{
    Michelson,
    data::Data,
    data,
    types::Type,
    types
};
use tezos_michelson::micheline::Micheline;
use tezos_core::{
    types::encoded::{ScriptExprHash, Encoded},
    internal::crypto::blake2b
};

use crate::{
    Result,
    types::{BigMapItem, StackItem, MapItem, BigMapPtr, OptionItem},
    typechecker::check_types_equal,
    interpreter::TransactionContext,
    err_type,
};

pub fn script_expr_hash(item: &StackItem, ty: &Type) -> Result<ScriptExprHash> {
    let expr = item.clone().into_micheline(&ty)?;
    let schema: Micheline = Michelson::from(ty.clone()).into();
    let payload = expr.pack(Some(&schema))?;
    let hash = blake2b(payload.as_slice(), 32)?;
    let res = ScriptExprHash::from_bytes(hash.as_slice())?;
    Ok(res)
}

impl BigMapPtr {
    pub fn new(ptr: i64, key_type: Type, val_type: Type) -> Self {
        Self { ptr, inner_type: (key_type, val_type), diff: Vec::new() }
    }
}

impl BigMapItem {
    pub fn from_data(data: Data, key_type: &Type, val_type: &Type) -> Result<StackItem> {
        match data {
            Data::Int(ptr) => {
                let lazy = BigMapPtr::new(ptr.to_integer()?, key_type.clone(), val_type.clone());
                Ok(StackItem::BigMap(Self::Ptr(lazy)))
            },
            Data::Sequence(sequence) => {
                let map = MapItem::from_sequence(sequence, key_type.clone(), val_type.clone())?;
                Ok(StackItem::BigMap(Self::Map(map)))
            },
            _ => err_type!("Data::Int or Data::Sequence", data)
        }
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        if let Type::BigMap(big_map_ty) = ty {
            return match self {
                Self::Ptr(lazy) => {
                    check_types_equal(&big_map_ty.key_type, &lazy.inner_type.0)?;
                    check_types_equal(&big_map_ty.value_type, &lazy.inner_type.1)?;
                    Ok(Data::Int(data::int(lazy.ptr)))
                },
                Self::Map(_) => err_type!(ty, self)  // NOTE: not supported
            }
        }
        err_type!(ty, self)
    }

    pub fn get_type(&self) -> Result<Type> {
        match self {
            Self::Ptr(lazy) => {
                Ok(types::big_map(lazy.inner_type.0.clone(), lazy.inner_type.1.clone()))
            },
            Self::Map(map) => map.get_type()
        }
    }

    pub fn contains(&self, key: &StackItem, context: &impl TransactionContext) -> Result<bool> {
        match self {
            Self::Map(map) => map.contains(key),
            Self::Ptr(lazy) => {
                let key_hash = script_expr_hash(key, &lazy.inner_type.0)?;
                context.has_big_map_value(lazy.ptr, &key_hash)
            }
        }
    }

    pub fn get(&self, key: &StackItem, context: &impl TransactionContext) -> Result<OptionItem> {
        match self {
            Self::Map(map) => map.get(key),
            Self::Ptr(lazy) => {
                let key_hash = script_expr_hash(key, &lazy.inner_type.0)?;
                match context.get_big_map_value(lazy.ptr, &key_hash)? {
                    Some(val) => {
                        let item = StackItem::from_micheline(val, &lazy.inner_type.1)?;
                        OptionItem::some(item)
                    },
                    None => Ok(OptionItem::none(&lazy.inner_type.1))
                }
            }
        }
    }

    pub fn update(&mut self, key: StackItem, val: Option<StackItem>, context: &mut impl TransactionContext) -> Result<OptionItem> {
        match self {
            Self::Map(map) => {
                let old_val = map.update(key, val)?;
                Ok(old_val)
            },
            Self::Ptr(lazy) => {
                let key_hash = script_expr_hash(&key, &lazy.inner_type.0)?;
                let value = match val {
                    Some(val) => Some(val.into_micheline(&lazy.inner_type.1)?),
                    None => None
                };
                match context.set_big_map_value(lazy.ptr, key_hash, value)? {
                    Some(old_val) => {
                        let item = StackItem::from_micheline(old_val, &lazy.inner_type.1)?;
                        OptionItem::some(item)
                    },
                    None => Ok(OptionItem::none(&lazy.inner_type.1))
                }
            }
        }
    }
}

impl Display for BigMapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Map(map) => map.fmt(f),
            Self::Ptr(lazy) => f.write_fmt(format_args!("${}", lazy.ptr))
        }
    }
}