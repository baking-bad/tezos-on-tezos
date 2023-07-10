use crate::{Error, Result};
use tezos_michelson::michelson::{
    data::{self, Data},
    types::{self, Type},
};

pub fn create_pair(inner_types: Vec<Type>, field_name: Option<String>) -> Type {
    let ty = tezos_michelson::michelson::types::Pair::new(inner_types, None);
    match field_name {
        Some(value) => ty.with_field_annotation(value),
        None => ty.into(),
    }
}

pub fn create_unit(field_name: Option<String>) -> Type {
    let ty = tezos_michelson::michelson::types::Unit::new(None);
    match field_name {
        Some(value) => ty.with_field_annotation(value),
        None => ty.into(),
    }
}

pub fn flatten_pair(ty: Type, data: Data) -> Result<data::Pair> {
    let pair_ty: types::Pair = ty.try_into()?;
    let pair: data::Pair = data.try_into()?;

    let mut args = pair.values;

    for (i, arg_type) in pair_ty.types.iter().enumerate() {
        if i >= args.len() {
            return Err(Error::TypeMismatch {
                message: format!(
                    "Expected pair with {} fields, got only {}",
                    pair_ty.types.len(),
                    args.len()
                ),
            });
        }

        while let Data::Pair(_) = &args[i] {
            if let Type::Pair(_) = arg_type {
                // Named inner pair does not collapse
                break;
            } else {
                let inner_pair: data::Pair = args.remove(i).try_into()?;
                for (j, inner_arg) in inner_pair.values.into_iter().enumerate() {
                    args.insert(i + j, inner_arg)
                }
            }
        }
    }

    Ok(data::pair(args))
}

// Right-trees supported only (combs-like)
//    or
//   /  \
//  0    or
//      /  \
//     1    or
//         /  \
//        2    3
// WARNING: panics if the size of inner args is less than two
pub fn make_nested_or(inner_types: Vec<Type>) -> types::Or {
    let mut inner_types = inner_types;
    let lhs = inner_types.remove(0);
    let rhs = match inner_types.len() {
        0 => unreachable!("Invalid number of enum args (must be >= 2)"),
        1 => inner_types.remove(0),
        _ => make_nested_or(inner_types).into(),
    };
    types::or(lhs, rhs)
}

// Extended node ordering
//    0
//   / \
//  1   2
//     / \
//    3   4
//       / \
//      5   6
fn wrap_or(data: Data, node_idx: usize) -> Data {
    if node_idx == 0 {
        return data;
    }
    if node_idx % 2 == 0 {
        wrap_or(data::right(data), node_idx - 2)
    } else {
        wrap_or(data::left(data), node_idx - 1)
    }
}

pub fn wrap_or_variant(data: Data, var_idx: usize, var_count: usize) -> Data {
    let node_idx = if var_idx == var_count - 1 {
        2 * var_idx
    } else {
        2 * var_idx + 1
    };
    wrap_or(data, node_idx)
}

fn unwrap_or(data: Data, node_idx: usize) -> (Data, usize) {
    match data {
        Data::Left(lhs) => return (*lhs.value, node_idx + 1),
        Data::Right(rhs) => return unwrap_or(*rhs.value, node_idx + 2),
        leaf => return (leaf, node_idx),
    }
}

// TODO: can panic in runtime?
pub fn unwrap_or_variant(data: Data, var_count: usize) -> (Data, usize) {
    let (res, node_idx) = unwrap_or(data, 0);
    let var_idx = if node_idx == 2 * (var_count - 1) {
        var_count - 1
    } else {
        (node_idx - 1) / 2
    };
    (res, var_idx)
}
