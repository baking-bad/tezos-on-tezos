pub mod inbox;
pub mod raw;

pub use crate::context::inbox::{read_inbox, InboxMessage};
pub use crate::context::raw::{storage_backup_n_write, storage_clear_backup};

use std::collections::{HashMap, HashSet};
use host::{
    path::RefPath,
    runtime::{Runtime, ValueType}
};
use tez_proto::{
    context::{Context, types::{ContextNode, ContextNodeType}},
    error::Result
};

fn err_into(e: impl std::fmt::Debug) -> tez_proto::error::Error {
    tez_proto::error::Error::InternalError {
        kind: tez_proto::error::ErrorKind::Context,
        message: format!("{:?}", e)
    }
}

pub struct PVMContext<Host> where Host: Runtime {
    host: Host,
    state: HashMap<String, ContextNode>,
    modified_keys: HashSet<String>
}

impl<Host> AsMut<Host> for PVMContext<Host> where Host: Runtime {
    fn as_mut(&mut self) -> &mut Host {
        &mut self.host
    }
}

impl<Host> PVMContext<Host> where Host: Runtime {
    pub fn new(host: Host) -> Self {
        PVMContext {
            host,
            state: HashMap::new(),
            modified_keys: HashSet::new()
        }
    }
}

impl<Host> Context for PVMContext<Host> where Host: Runtime {
    fn has(&self, key: String) -> Result<bool> {
        match self.state.contains_key(&key) {
            true => Ok(true),
            false => {
                let path = RefPath::assert_from(key.as_bytes());
                match self.host.store_has(&path) {
                    Ok(Some(ValueType::Value)) => Ok(true),
                    Err(err) => Err(err_into(err)),
                    _ => Ok(false)
                }
            }
        }
    }

    fn get<V: ContextNodeType>(&mut self, key: String, max_bytes: usize) -> Result<Option<V>> {
        match self.state.get(&key) {
            Some(cached_value) => Ok(Some(V::unwrap(cached_value.to_owned()))),
            None => {
                let path = RefPath::assert_from(key.as_bytes());
                match self.host.store_has(&path) {
                    Ok(Some(ValueType::Value)) => {
                        let stored_value = self.host
                            .store_read(&path, 0, max_bytes)
                            .map_err(err_into)?;
                        let value = V::decode(&stored_value)?;
                        let inner_value = V::unwrap(value.to_owned());
                        self.state.insert(key, value);
                        Ok(Some(inner_value))
                    },
                    Ok(Some(node_type)) => Err(err_into(node_type)),
                    Ok(None) => Ok(None),
                    Err(err) => Err(err_into(err))
                }
            }
        }
    }

    fn set<V: ContextNodeType>(&mut self, key: String, val: V) -> Result<()> {
        self.state.insert(key.clone(), val.wrap());
        self.modified_keys.insert(key);
        Ok(())
    }

    fn persist<V: ContextNodeType>(&mut self, key: String, val: V) -> Result<()> {
        let raw_value = val.encode()?;
        let path = RefPath::assert_from(key.as_bytes());
        storage_backup_n_write(&mut self.host, &path, raw_value.as_slice()).map_err(err_into)?;
        self.state.insert(key, val.wrap());
        Ok(())
    }

    fn has_pending_changes(&self) -> bool {
        !self.modified_keys.is_empty()
    }

    fn commit(&mut self) -> Result<()> {
        let mut checksum = self.get_checksum()?;

        for key in self.modified_keys.iter() {
            let cached_value = self.state.get(key).expect("Modified value expected to be in state");
            let raw_value = cached_value.to_vec()?;
            let path = RefPath::assert_from(key.as_bytes());
            storage_backup_n_write(&mut self.host, &path, raw_value.as_slice()).map_err(err_into)?;
            checksum.update(&raw_value)?;
        }

        self.persist("/context/checksum".into(), checksum)?;
        self.modified_keys.clear();
        Ok(())
    }

    fn rollback(&mut self) {
        for key in self.modified_keys.iter() {
            self.state.remove(key);
        }
        self.modified_keys.clear();
    }

    fn clear(&mut self) {
        self.state.clear();
        self.modified_keys.clear();
        storage_clear_backup(&mut self.host);  // PathNotFound is OK
    }
}

#[cfg(test)]
mod test {
    use mock_runtime::host::MockHost;
    use tez_proto::context::Context;
    use tez_proto::error::Result;
    use crate::context::PVMContext;

    #[test]
    fn store_balance() -> Result<()> {
        let mut context = PVMContext::new(MockHost::default());

        let address = "tz1Mj7RzPmMAqDUNFBn5t5VbXmWW4cSUAdtT";
        let balance = 1000u32.into();

        assert!(context.get_balance(&address)?.is_none());  // both host and cache accessed

        context.set_balance(&address, &balance)?;  // cached
        context.commit()?;  // sent to the host
        context.clear();  // cache cleared

        assert!(context.get_balance(&address)?.is_some());  // cached
        assert_eq!(context.get_balance(&address)?.expect("Balance must not be null"), balance);  // served from the cache

        Ok(())
    }
}