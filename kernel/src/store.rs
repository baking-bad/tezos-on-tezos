use host::{
    path::{RefPath},
    runtime::{Runtime, RuntimeError, ValueType, load_value_sized, save_value_sized},
    rollup_core::RawRollupCore
};

fn err_into(e: impl std::fmt::Debug) -> context::Error {
    context::Error::Internal(context::error::InternalError::new(
        context::error::InternalKind::Store,
        format!("PVM context error: {:?}", e),
    ))
}

macro_rules! str_to_path {
    ($key: expr) => {
        RefPath::assert_from($key.as_bytes())
    };
}

pub fn store_has(host: &impl RawRollupCore, key: &str) -> context::Result<bool> {
    match Runtime::store_has(host, &str_to_path!(key)) {
        Ok(Some(ValueType::Value)) => Ok(true),
        Err(err) => Err(err_into(err)),
        _ => Ok(false),
    }
}

pub fn store_read(host: &impl RawRollupCore, key: &str) -> context::Result<Option<Vec<u8>>> {
    match load_value_sized(host, &str_to_path!(key)) {
        Ok(val) => Ok(Some(val)),
        Err(RuntimeError::PathNotFound) => Ok(None),
        Err(err) => Err(err_into(err))
    }
}

pub fn store_write(host: &mut impl RawRollupCore, key: &str, val: Vec<u8>) -> context::Result<()> {
    save_value_sized(host, &str_to_path!(key), val.as_slice());  // TODO(kernel): expose error instead of panic?
    Ok(())
}

pub fn store_delete(host: &mut impl RawRollupCore, key: &str) -> context::Result<()> {
    match Runtime::store_delete(host, &str_to_path!(key)) {
        Ok(()) => Ok(()),
        Err(RuntimeError::PathNotFound) => Ok(()),
        Err(err) => Err(err_into(err))
    }
}

pub fn store_move(host: &mut impl RawRollupCore, from_key: &str, to_key: &str) -> context::Result<()> {
    Runtime::store_move(host, &str_to_path!(from_key), &str_to_path!(to_key)).map_err(err_into)
}