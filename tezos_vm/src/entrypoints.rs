use std::collections::HashMap;
use tezos_michelson::micheline::{primitive_application::PrimitiveApplication, Micheline};
use tezos_michelson::michelson::types::Type;

use crate::{Error, Result};

// Initial mask value: 0b1 <-- terminating 1
// Resulting mask value (example): 0b10001010010
// Least significant bit indicates the entrypoint node
// In order to wrap parameter with "Left"s and "Right"s one need to go in reverse order
pub fn get_entrypoint_mask(entrypoint: &str, ty: &Type, mask: i32) -> Result<i32> {
    if let Some(annot) = ty.metadata().field_name() {
        if annot.value_without_prefix() == entrypoint {
            return Ok(mask);
        }
    }
    if let Type::Or(or) = ty.clone() {
        if let Ok(res) = get_entrypoint_mask(entrypoint, &or.lhs, mask << 1) {
            return Ok(res);
        }
        if let Ok(res) = get_entrypoint_mask(entrypoint, &or.rhs, (mask << 1) | 1) {
            return Ok(res);
        }
    }
    if mask == 1 && entrypoint == "default" {
        return Ok(mask);
    }
    Err(Error::EntrypointNotFound {
        name: entrypoint.into(),
    })
}

pub fn normalize_parameter(
    parameter: Micheline,
    entrypoint: &str,
    param_ty: &Type,
) -> Result<Micheline> {
    let mut parameter = parameter.normalized();
    let mut mask = get_entrypoint_mask(entrypoint, param_ty, 1)?;
    assert!(mask > 0);

    while mask > 1 {
        let prim: String = if mask & 1 == 0 {
            "Left".into()
        } else {
            "Right".into()
        };
        parameter = PrimitiveApplication::new(prim, Some(vec![parameter]), None).into();
        mask >>= 1;
    }

    Ok(parameter)
}

pub fn search_entrypoint(ty: Type, entrypoint: Option<&str>, depth: usize) -> Result<Type> {
    let entrypoint = entrypoint.unwrap_or("default");
    if let Some(annot) = ty.metadata().field_name() {
        if annot.value_without_prefix() == entrypoint {
            return Ok(ty);
        }
    }
    if let Type::Or(or) = ty.clone() {
        if let Ok(ty) = search_entrypoint(*or.lhs, Some(entrypoint), depth + 1) {
            return Ok(ty);
        }
        if let Ok(ty) = search_entrypoint(*or.rhs, Some(entrypoint), depth + 1) {
            return Ok(ty);
        }
    }
    if depth == 0 && entrypoint == "default" {
        return Ok(ty);
    }
    Err(Error::EntrypointNotFound {
        name: entrypoint.into(),
    })
}

pub fn collect_entrypoints(ty: Type, res: &mut HashMap<String, Type>, depth: usize) -> Result<()> {
    if let Some(annot) = ty.metadata().field_name() {
        let name = annot.value_without_prefix();
        if res.contains_key(name) {
            return Err(Error::ConflictingEntrypoints { address: "".into() });
        }
        res.insert(name.into(), ty.clone());
    }
    if let Type::Or(or) = ty.clone() {
        collect_entrypoints(*or.lhs, res, depth + 1)?;
        collect_entrypoints(*or.rhs, res, depth + 1)?;
    }
    if depth == 0 && !res.contains_key("default") {
        res.insert("default".into(), ty);
    }
    Ok(())
}
