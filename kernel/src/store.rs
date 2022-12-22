
use host::{
    path::{RefPath, concat},
    runtime::{Runtime, RuntimeError}
};
use crate::error::Result;

macro_rules! rew_prefix {
    ($lvl: expr) => {
        RefPath::assert_from(format!("/rewind/{}", $lvl).as_bytes())
    };
}

pub fn store_move_write(host: &mut impl Runtime, path: &RefPath, value: &[u8], level: i32) -> Result<()> {
    if level >= 0 {
        // NOTE: keep the original version (can be both value and subtree in case of removal)
        let rew_path = concat(&rew_prefix!(level), path)?;
        if None == host.store_has(&rew_path)? {
            match host.store_move(path, &rew_path) {
                Ok(_) => (),
                Err(RuntimeError::PathNotFound) => {
                    host.store_write(&rew_path, b"", 0)?;
                },
                Err(err) => return Err(err.into())
            }
        }
    }
    host.store_write(path, value, 0)?;  // FIXME: possible trailing garbage
    Ok(())
}

pub fn store_prune(host: &mut impl Runtime, level: i32) -> Result<bool> {
    match host.store_delete(&rew_prefix!(level)) {
        Ok(_) => Ok(true),
        Err(RuntimeError::PathNotFound) => Ok(false),
        Err(err) => Err(err.into())
    }
}

pub fn store_rewind(_host: &mut impl Runtime, _level: i32) {
    todo!("Unroll terminating leaves, delete null subtrees, and remove level")
}

#[cfg(test)]
mod test {
    use host::runtime::Runtime;
    use mock_runtime::host::MockHost;
    use crate::error::Result;
    use super::*;

    #[test]
    fn test_rewind() -> Result<()> {
        let mut host = MockHost::default();
        let path = RefPath::assert_from(b"/test");

        host.store_write(&path, b"hello", 0)?;
        
        store_move_write(&mut host, &path, b"hi", 0)?;

        let rew_path = concat(&rew_prefix!(0), &path)?;
        let res = host.store_has(&rew_path)?;

        assert!(res.is_some());
        Ok(())
    }
}