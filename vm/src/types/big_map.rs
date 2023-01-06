use std::fmt::Display;
use std::collections::BTreeMap;
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
    Error,
    types::{BigMapItem, StackItem, MapItem, BigMapPtr, OptionItem, OrItem, ListItem, PairItem},
    typechecker::check_types_equal,
    interpreter::{TransactionContext, TransactionScope, LazyStorage},
    err_type,
};

pub fn script_expr_hash(expr: Micheline, ty: &Type) -> Result<ScriptExprHash> {
    let schema: Micheline = Michelson::from(ty.clone()).into();
    let payload = expr.pack(Some(&schema))?;
    let hash = blake2b(payload.as_slice(), 32)?;
    let res = ScriptExprHash::from_bytes(hash.as_slice())?;
    Ok(res)
}

pub fn get_key_hash(key: &StackItem, ty: &Type) -> Result<ScriptExprHash> {
    let expr = key.clone().into_micheline(&ty)?;
    script_expr_hash(expr, ty)
}

pub fn check_ownership(ptr: i64, scope: &TransactionScope, context: &mut impl TransactionContext) -> Result<()> {
    let owner = context.get_big_map_owner(ptr)?;
    if owner == scope.self_address {
        Ok(())
    } else {
        Err(Error::BigMapAccessDenied {
            ptr, owner:
            owner.into_string(),
            offender: scope.self_address.clone().into_string()
        })
    }
}

impl BigMapPtr {
    pub fn new(ptr: i64, key_type: Type, val_type: Type) -> Self {
        Self { ptr, inner_type: (key_type, val_type), diff: BTreeMap::new(), new: true }
    }

    pub fn update(&mut self, key_hash: String, key: Micheline, val: Option<Micheline>) {
        self.diff.insert(key_hash, (key, val));
    }

    pub fn value(&self) -> i64 {
        self.ptr
    }
}

impl BigMapItem {
    pub fn from_data(data: Data, key_type: &Type, val_type: &Type) -> Result<StackItem> {
        match data {
            Data::Int(ptr) => {
                let lazy = BigMapPtr {
                    ptr: ptr.to_integer()?,
                    inner_type: (key_type.clone(), val_type.clone()),
                    diff: BTreeMap::new(),
                    new: false
                };
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
                let key_hash = get_key_hash(key, &lazy.inner_type.0)?;
                context.has_big_map_value(lazy.ptr, &key_hash)
            }
        }
    }

    pub fn get(&self, key: &StackItem, context: &impl TransactionContext) -> Result<OptionItem> {
        match self {
            Self::Map(map) => map.get(key),
            Self::Ptr(lazy) => {
                let key_hash = get_key_hash(key, &lazy.inner_type.0)?;
                match context.get_big_map_value(lazy.ptr, &key_hash)? {
                    Some(val) => {
                        let item = StackItem::from_micheline(val, &lazy.inner_type.1)?;
                        Ok(OptionItem::some(item))
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
                let key_expr = key.into_micheline(&lazy.inner_type.0)?;
                let key_hash = script_expr_hash(key_expr.clone(), &lazy.inner_type.0)?;
                let val_expr = match val {
                    Some(val) => Some(val.into_micheline(&lazy.inner_type.1)?),
                    None => None
                };
                lazy.update(key_hash.into_string(), key_expr, val_expr.clone());
                match context.set_big_map_value(lazy.ptr, key_hash, val_expr)? {
                    Some(old_val) => {
                        let item = StackItem::from_micheline(old_val, &lazy.inner_type.1)?;
                        Ok(OptionItem::some(item))
                    },
                    None => Ok(OptionItem::none(&lazy.inner_type.1))
                }
            }
        }
    }
 
    pub fn acquire(self, scope: &TransactionScope, context: &mut impl TransactionContext) -> Result<Self> {
        match self {
            Self::Ptr(ref lazy) => {
                check_ownership(lazy.ptr, scope, context)?;
                Ok(self)
            },
            Self::Map(map) => {
                let ptr = context.allocate_big_map(scope.self_address.clone())?;
                let mut lazy = BigMapPtr::new(ptr, map.inner_type.0.clone(), map.inner_type.1.clone());
                for (key, val) in map.outer_value.clone() {
                    let key_expr = key.into_micheline(&lazy.inner_type.0)?;
                    let val_expr = val.into_micheline(&lazy.inner_type.1)?;
                    let key_hash = script_expr_hash(key_expr.clone(), &lazy.inner_type.0)?;
                    lazy.update(key_hash.into_string(), key_expr, Some(val_expr.clone()));
                    context.set_big_map_value(lazy.ptr, key_hash, Some(val_expr))?;
                }
                Ok(Self::Ptr(lazy))
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

impl PartialEq for BigMapPtr {
    fn eq(&self, other: &Self) -> bool {
        // for testing purposes only (ignoring pointer and types)
        self.diff == other.diff
    }
}

impl LazyStorage for BigMapItem {
    fn try_acquire(&mut self, scope: &TransactionScope, context: &mut impl TransactionContext) -> Result<()> {
        match self {
            Self::Ptr(lazy) => check_ownership(lazy.ptr, scope, context),
            Self::Map(_) => {
                *self = self.clone().acquire(scope, context)?;
                Ok(())
            }
        }
    }
}

impl LazyStorage for OptionItem {
    fn try_acquire(&mut self, scope: &TransactionScope, context: &mut impl TransactionContext) -> Result<()> {
        match self {
            Self::None(_) => Ok(()),
            Self::Some(val) => val.try_acquire(scope, context)
        }
    }
}

impl LazyStorage for OrItem {
    fn try_acquire(&mut self, scope: &TransactionScope, context: &mut impl TransactionContext) -> Result<()> {
        let var = match self {
            Self::Left(var) => var,
            Self::Right(var) => var
        };
        var.value.try_acquire(scope, context)
    }
}

impl LazyStorage for PairItem {
    fn try_acquire(&mut self, scope: &TransactionScope, context: &mut impl TransactionContext) -> Result<()> {
        self.0.0.try_acquire(scope, context)?;
        self.0.1.try_acquire(scope, context)
    }
}

impl LazyStorage for ListItem {
    fn try_acquire(&mut self, scope: &TransactionScope, context: &mut impl TransactionContext) -> Result<()> {
        self.outer_value
            .iter_mut()
            .map(|e| e.try_acquire(scope, context))
            .collect()
    }
}

impl LazyStorage for MapItem {
    fn try_acquire(&mut self, scope: &TransactionScope, context: &mut impl TransactionContext) -> Result<()> {
        self.outer_value
            .iter_mut()
            .map(|(_, v)| v.try_acquire(scope, context))
            .collect()
    }
}
