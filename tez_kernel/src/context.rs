use std::collections::HashMap;
use host::{
    path::RefPath,
    runtime::{Runtime, ValueType}
};
use tez_proto::context::{Context, node::{ContextNode, NodeType}};
use tez_proto::error::Result;

macro_rules! storage_error {
    ($($arg:tt)*) => {
        tez_proto::error::Error::StorageError { message: format!($($arg)*) }
    };
}

pub struct PVMContext<Host> where Host: Runtime {
    host: Host,
    state: HashMap<String, ContextNode>,
    modified_keys: Vec<String>
}

impl<Host> AsMut<Host> for PVMContext<Host> where Host: Runtime {
    fn as_mut(&mut self) -> &mut Host {
        &mut self.host
    }
}

impl<Host> AsRef<Host> for PVMContext<Host> where Host: Runtime {
    fn as_ref(&self) -> &Host {
        &self.host
    }
}

impl<Host> PVMContext<Host> where Host: Runtime {
    pub fn new(host: Host) -> Self {
        PVMContext {
            host,
            state: HashMap::new(),
            modified_keys: Vec::new()
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
                    Err(error) => Err(storage_error!("{:?}", error)),
                    _ => Ok(false)
                }
            }
        }
    }

    fn get<V: NodeType>(&mut self, key: String, max_bytes: usize) -> Result<Option<V>> {
        match self.state.get(&key) {
            Some(cached_value) => Ok(Some(V::unwrap(cached_value))),
            None => {
                let path = RefPath::assert_from(key.as_bytes());
                match self.host.store_has(&path) {
                    Ok(Some(ValueType::Value)) => {
                        let stored_value = self.host
                            .store_read(&path, 0, max_bytes)
                            .map_err(|e| storage_error!("{:?}", e))?;
                        let value = V::parse(&stored_value)?;
                        let inner_value = V::unwrap(&value);
                        self.state.insert(key, value);
                        Ok(Some(inner_value))
                    },
                    Ok(Some(node_type)) => Err(storage_error!("Unexpected node type {:?}", node_type)),
                    Ok(None) => Ok(None),
                    Err(error) => Err(storage_error!("{:?}", error))
                }
            }
        }
    }

    fn set<V: NodeType>(&mut self, key: String, val: &V) -> Result<()> {
        self.state.insert(key.clone(), V::wrap(val));
        self.modified_keys.push(key);
        Ok(())
    }

    fn has_pending_changes(&self) -> bool {
        !self.modified_keys.is_empty()
    }

    fn commit(&mut self) -> Result<()> {
        for key in self.modified_keys.iter() {
            let path = RefPath::assert_from(key.as_bytes());
            let cached_value = self.state.get(key).expect("Modified value expected to be in state");
            let raw_value = cached_value.to_vec()?;
            // FIXME: trailing garbage?
            self.host
                .store_write(&path, raw_value.as_slice(), 0)
                .map_err(|e| storage_error!("{:?}", e))?;
        }
        self.modified_keys.clear();
        Ok(())
    }

    fn clear(&mut self) {
        self.state.clear();
    }

    fn rollback(&mut self) {
        for key in self.modified_keys.iter() {
            self.state.remove(key);
        }
        self.modified_keys.clear();
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