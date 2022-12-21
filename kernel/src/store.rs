
use host::{
    path::{RefPath, concat},
    runtime::{Runtime, RuntimeError}
};
use crate::error::Result;

pub fn store_move_write(host: &mut impl Runtime, path: &RefPath, value: &[u8]) -> Result<()> {
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

pub fn store_delete_backup(host: &mut impl Runtime) {
    match host.store_delete(&RefPath::assert_from(b"/backup")) {
        Ok(_) => (),
        Err(RuntimeError::PathNotFound) => (),
        Err(_) => todo!("Handle?")
    }
}

pub fn store_unroll_backup(_host: &mut impl Runtime) {
    todo!("Unroll terminating leaves, delete null subtrees, and remove /backup")
}
