use std::collections::BTreeMap;
use std::fmt::Display;
use tezos_core::{
    internal::crypto::blake2b,
    types::encoded::{ContractAddress, Encoded, ScriptExprHash},
};
use tezos_michelson::micheline::Micheline;
use tezos_michelson::michelson::{data, data::Data, types, types::Type, Michelson};

use crate::typechecker::check_pair_len;
use crate::{
    err_mismatch,
    formatter::Formatter,
    interpreter::{InterpreterContext, LazyStorage},
    type_cast,
    typechecker::check_types_equal,
    types::{BigMapDiff, BigMapItem, ListItem, MapItem, OptionItem, OrItem, PairItem, StackItem},
    Error, Result,
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

pub fn check_ownership(
    ptr: i64,
    owner: &ContractAddress,
    context: &mut impl InterpreterContext,
) -> Result<()> {
    let actual_owner = context.get_big_map_owner(ptr)?;
    if *owner == actual_owner {
        Ok(())
    } else {
        Err(Error::BigMapAccessDenied { ptr })
    }
}

impl BigMapDiff {
    pub fn new(ptr: i64, key_type: Type, val_type: Type) -> Self {
        Self {
            id: ptr,
            inner_type: (key_type, val_type),
            updates: BTreeMap::new(),
            alloc: true,
        }
    }

    pub fn update(&mut self, key_hash: String, key: Micheline, val: Option<Micheline>) {
        self.updates.insert(key_hash, (key, val));
    }

    pub fn value(&self) -> i64 {
        self.id
    }
}

impl BigMapItem {
    pub fn from_data(data: Data, key_type: &Type, val_type: &Type) -> Result<StackItem> {
        match data {
            Data::Int(ptr) => {
                let diff = BigMapDiff {
                    id: ptr.to_integer()?,
                    inner_type: (key_type.clone(), val_type.clone()),
                    updates: BTreeMap::new(),
                    alloc: false,
                };
                Ok(StackItem::BigMap(Self::Diff(diff)))
            }
            Data::Sequence(sequence) => {
                let map = MapItem::from_sequence(sequence, key_type.clone(), val_type.clone())?;
                Ok(StackItem::BigMap(Self::Map(map)))
            }
            Data::Map(elt_map) => {
                let map = MapItem::from_elt_map(elt_map, key_type.clone(), val_type.clone())?;
                Ok(StackItem::BigMap(Self::Map(map)))
            }
            _ => err_mismatch!("Data::Int or Data::Sequence", data.format()),
        }
    }

    pub fn into_data(self) -> Result<Data> {
        match self {
            Self::Ptr(id) => Ok(Data::Int(data::int(id))),
            _ => err_mismatch!("Ptr", self),
        }
    }

    pub fn get_type(&self) -> Result<Type> {
        match self {
            Self::Diff(diff) => Ok(types::big_map(
                diff.inner_type.0.clone(),
                diff.inner_type.1.clone(),
            )),
            Self::Map(map) => map.get_type(),
            Self::Ptr(_) => err_mismatch!("Diff or Map", self),
        }
    }

    pub fn contains(&self, key: &StackItem, context: &impl InterpreterContext) -> Result<bool> {
        match self {
            Self::Map(map) => map.contains(key),
            Self::Diff(diff) => {
                let key_hash = get_key_hash(key, &diff.inner_type.0)?;
                context.has_big_map_value(diff.id, &key_hash)
            }
            Self::Ptr(_) => err_mismatch!("Diff or Map", self),
        }
    }

    pub fn get(&self, key: &StackItem, context: &impl InterpreterContext) -> Result<OptionItem> {
        match self {
            Self::Map(map) => map.get(key),
            Self::Diff(diff) => {
                let key_hash = get_key_hash(key, &diff.inner_type.0)?;
                match context.get_big_map_value(diff.id, &key_hash)? {
                    Some(val) => {
                        let item = StackItem::from_micheline(val, &diff.inner_type.1)?;
                        Ok(OptionItem::some(item))
                    }
                    None => Ok(OptionItem::none(&diff.inner_type.1)),
                }
            }
            Self::Ptr(_) => err_mismatch!("Diff or Map", self),
        }
    }

    pub fn update(
        &mut self,
        key: StackItem,
        val: Option<StackItem>,
        context: &mut impl InterpreterContext,
    ) -> Result<OptionItem> {
        match self {
            Self::Map(map) => {
                let old_val = map.update(key, val)?;
                Ok(old_val)
            }
            Self::Diff(diff) => {
                let key_expr = key.into_micheline(&diff.inner_type.0)?;
                let key_hash = script_expr_hash(key_expr.clone(), &diff.inner_type.0)?;
                let val_expr = match val {
                    Some(val) => Some(val.into_micheline(&diff.inner_type.1)?),
                    None => None,
                };
                diff.update(key_hash.into_string(), key_expr, val_expr.clone());
                match context.set_big_map_value(diff.id, key_hash, val_expr)? {
                    Some(old_val) => {
                        let item = StackItem::from_micheline(old_val, &diff.inner_type.1)?;
                        Ok(OptionItem::some(item))
                    }
                    None => Ok(OptionItem::none(&diff.inner_type.1)),
                }
            }
            Self::Ptr(_) => err_mismatch!("Diff or Map", self),
        }
    }

    pub fn acquire(
        self,
        owner: &ContractAddress,
        context: &mut impl InterpreterContext,
    ) -> Result<Self> {
        match self {
            Self::Diff(ref diff) => {
                check_ownership(diff.id, owner, context)?;
                Ok(self)
            }
            Self::Map(map) => {
                let ptr = context.allocate_big_map(owner.clone())?;
                let mut diff =
                    BigMapDiff::new(ptr, map.inner_type.0.clone(), map.inner_type.1.clone());
                for (key, val) in map.outer_value.clone() {
                    let key_expr = key.into_micheline(&diff.inner_type.0)?;
                    let val_expr = val.into_micheline(&diff.inner_type.1)?;
                    let key_hash = script_expr_hash(key_expr.clone(), &diff.inner_type.0)?;
                    diff.update(key_hash.into_string(), key_expr, Some(val_expr.clone()));
                    context.set_big_map_value(diff.id, key_hash, Some(val_expr))?;
                }
                Ok(Self::Diff(diff))
            }
            Self::Ptr(_) => err_mismatch!("Diff or Map", self),
        }
    }
}

impl Display for BigMapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Map(map) => map.fmt(f),
            Self::Diff(diff) => f.write_fmt(format_args!("${}", diff.id)),
            Self::Ptr(id) => f.write_fmt(format_args!("${}", id)),
        }
    }
}

impl PartialEq for BigMapDiff {
    fn eq(&self, other: &Self) -> bool {
        // for testing purposes only (ignoring pointer and types)
        self.updates == other.updates
    }
}

impl LazyStorage for BigMapItem {
    fn try_acquire(
        &mut self,
        owner: &ContractAddress,
        context: &mut impl InterpreterContext,
    ) -> Result<()> {
        match self {
            Self::Diff(diff) => check_ownership(diff.id, owner, context),
            Self::Map(_) => {
                *self = self.clone().acquire(owner, context)?;
                Ok(())
            }
            Self::Ptr(_) => err_mismatch!("Diff or Map", self),
        }
    }

    fn try_aggregate(&mut self, output: &mut Vec<BigMapDiff>, ty: &Type) -> Result<()> {
        match self {
            Self::Diff(diff) => {
                let big_map_ty = type_cast!(ty, BigMap);
                check_types_equal(&big_map_ty.key_type, &diff.inner_type.0)?;
                check_types_equal(&big_map_ty.value_type, &diff.inner_type.1)?;
                output.push(diff.clone());
                *self = Self::Ptr(diff.id);
                Ok(())
            }
            _ => err_mismatch!("Diff", self),
        }
    }
}

impl LazyStorage for OptionItem {
    fn try_acquire(
        &mut self,
        owner: &ContractAddress,
        context: &mut impl InterpreterContext,
    ) -> Result<()> {
        match self {
            Self::None(_) => Ok(()),
            Self::Some(val) => val.try_acquire(owner, context),
        }
    }

    fn try_aggregate(&mut self, output: &mut Vec<BigMapDiff>, ty: &Type) -> Result<()> {
        match self {
            Self::None(_) => Ok(()),
            Self::Some(val) => {
                let ty = type_cast!(ty, Option);
                val.try_aggregate(output, &ty.r#type)
            }
        }
    }
}

impl LazyStorage for OrItem {
    fn try_acquire(
        &mut self,
        owner: &ContractAddress,
        context: &mut impl InterpreterContext,
    ) -> Result<()> {
        let var = match self {
            Self::Left(var) => var,
            Self::Right(var) => var,
        };
        var.value.try_acquire(owner, context)
    }

    fn try_aggregate(&mut self, output: &mut Vec<BigMapDiff>, ty: &Type) -> Result<()> {
        let ty = type_cast!(ty, Or);
        let (var, ty) = match self {
            Self::Left(var) => (var, &ty.lhs),
            Self::Right(var) => (var, &ty.rhs),
        };
        var.value.try_aggregate(output, ty)
    }
}

impl LazyStorage for PairItem {
    fn try_acquire(
        &mut self,
        owner: &ContractAddress,
        context: &mut impl InterpreterContext,
    ) -> Result<()> {
        self.0 .0.try_acquire(owner, context)?;
        self.0 .1.try_acquire(owner, context)
    }

    fn try_aggregate(&mut self, output: &mut Vec<BigMapDiff>, ty: &Type) -> Result<()> {
        let ty = type_cast!(ty, Pair);
        check_pair_len(ty.types.len())?;
        self.0 .0.try_aggregate(output, &ty.types[0])?;
        self.0 .1.try_aggregate(output, &ty.types[1])
    }
}

impl LazyStorage for ListItem {
    fn try_acquire(
        &mut self,
        owner: &ContractAddress,
        context: &mut impl InterpreterContext,
    ) -> Result<()> {
        self.outer_value
            .iter_mut()
            .map(|e| e.try_acquire(owner, context))
            .collect()
    }

    fn try_aggregate(&mut self, output: &mut Vec<BigMapDiff>, ty: &Type) -> Result<()> {
        let ty = type_cast!(ty, List);
        self.outer_value
            .iter_mut()
            .map(|e| e.try_aggregate(output, &ty.r#type))
            .collect()
    }
}

impl LazyStorage for MapItem {
    fn try_acquire(
        &mut self,
        owner: &ContractAddress,
        context: &mut impl InterpreterContext,
    ) -> Result<()> {
        self.outer_value
            .iter_mut()
            .map(|(_, v)| v.try_acquire(owner, context))
            .collect()
    }

    fn try_aggregate(&mut self, output: &mut Vec<BigMapDiff>, ty: &Type) -> Result<()> {
        let ty = type_cast!(ty, Map);
        self.outer_value
            .iter_mut()
            .map(|(_, v)| v.try_aggregate(output, &ty.value_type))
            .collect()
    }
}
