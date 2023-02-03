use host::{
    path::RefPath,
    rollup_core::RawRollupCore,
    runtime::{load_value_sized, save_value_sized, Runtime, RuntimeError, ValueType},
};

fn err_into(e: impl std::fmt::Debug) -> tezos_ctx::Error {
    tezos_ctx::Error::Internal(tezos_ctx::error::InternalError::new(
        tezos_ctx::error::InternalKind::Store,
        format!("PVM context error: {:?}", e),
    ))
}

macro_rules! str_to_path {
    ($key: expr) => {
        RefPath::assert_from($key.as_bytes())
    };
}

pub fn store_has(host: &impl RawRollupCore, key: &str) -> tezos_ctx::Result<bool> {
    match Runtime::store_has(host, &str_to_path!(key)) {
        Ok(Some(ValueType::Value)) => Ok(true),
        Err(err) => Err(err_into(err)),
        _ => Ok(false),
    }
}

pub fn store_read(host: &impl RawRollupCore, key: &str) -> tezos_ctx::Result<Option<Vec<u8>>> {
    match load_value_sized(host, &str_to_path!(key)) {
        Ok(val) => Ok(Some(val)),
        Err(RuntimeError::PathNotFound) => Ok(None),
        Err(err) => Err(err_into(err)),
    }
}

pub fn store_write(
    host: &mut impl RawRollupCore,
    key: &str,
    val: Vec<u8>,
) -> tezos_ctx::Result<()> {
    save_value_sized(host, &str_to_path!(key), val.as_slice()); // TODO(kernel): expose error instead of panic?
    Ok(())
}

pub fn store_delete(host: &mut impl RawRollupCore, key: &str) -> tezos_ctx::Result<()> {
    match Runtime::store_delete(host, &str_to_path!(key)) {
        Ok(()) => Ok(()),
        Err(RuntimeError::PathNotFound) => Ok(()),
        Err(err) => Err(err_into(err)),
    }
}

pub fn store_move(
    host: &mut impl RawRollupCore,
    from_key: &str,
    to_key: &str,
) -> tezos_ctx::Result<()> {
    Runtime::store_move(host, &str_to_path!(from_key), &str_to_path!(to_key)).map_err(err_into)
}
