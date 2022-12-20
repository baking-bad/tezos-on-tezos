
use host::{
    path::{RefPath, concat},
    runtime::{Runtime, RuntimeError}
};
use crate::error::Result;

pub fn storage_backup_n_write(host: &mut impl Runtime, path: &RefPath, value: &[u8]) -> Result<()> {
    let backup_path = concat(&RefPath::assert_from(b"/backup"), path)?;
    // NOTE: keep the original version (can be both value and subtree in case of removal)
    if None == host.store_has(&backup_path)? {
        match host.store_move(path, &backup_path) {
            Ok(_) => (),
            Err(RuntimeError::PathNotFound) => {
                let backup_path = concat(&backup_path, &RefPath::assert_from(b"/null"))?;
                host.store_write(&backup_path, b"", 0)?;
            },
            Err(err) => return Err(err.into())
        }
    }
    host.store_write(path, value, 0)?;  // FIXME: possible trailing garbage
    Ok(())
}

pub fn storage_clear_backup(host: &mut impl Runtime) -> Result<()> {
    host.store_delete(&RefPath::assert_from(b"/backup")).map_err(|e| e.into())
}

pub fn storage_unroll(_host: &mut impl Runtime) {
    todo!("Unroll terminating leaves, delete null subtrees, and remove /backup")
}

pub fn debug_log(message: String) {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        host::rollup_core::write_debug(message.as_ptr(), message.len())
    };
    #[cfg(not(target_arch = "wasm32"))]
    {
        eprintln!("[DEBUG] {}", message);
    };
}

#[macro_export]
macro_rules! debug_msg {
    ($($arg:tt)*) => {
        crate::context::raw::debug_log(format!($($arg)*))
    };
}