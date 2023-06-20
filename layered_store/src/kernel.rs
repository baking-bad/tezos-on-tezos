use tezos_smart_rollup::{host::{Runtime, RuntimeError}, storage::path::RefPath};
use std::collections::{HashMap, HashSet};

use crate::{
    StoreType, LayeredStore, Result,
};

const TMP_PREFIX: &str = "/tmp";

macro_rules! str_to_path {
    ($key: expr) => {
        RefPath::assert_from($key.as_bytes())
    };
}

pub struct KernelStore<'rt, Host, T>
where
    Host: Runtime, T: StoreType
{
    host: &'rt mut Host,
    state: HashMap<String, Option<T>>,
    modified_keys: HashSet<String>,
    saved_state: HashMap<String, bool>,
}

impl<'rt, Host, T> KernelStore<'rt, Host, T>
where
    Host: Runtime, T: StoreType
{
    pub fn new(host: &'rt mut Host) -> Self {
        KernelStore {
            host,
            state: HashMap::new(),
            saved_state: HashMap::new(),
            modified_keys: HashSet::new(),
        }
    }

    pub fn persist(&mut self) -> Result<()> {
        for (key, exists) in self.saved_state.drain() {
            if exists {
                self.host.store_move(&str_to_path!([TMP_PREFIX, &key].concat().as_str()), &str_to_path!(&key))?;
            } else {
                self.host.store_delete(&str_to_path!(&key))?;
            }
        }
        Ok(())
    }

    pub fn as_host(&mut self) -> &mut Host {
        &mut self.host
    }
}

impl<'rt, Host, T> LayeredStore<T> for KernelStore<'rt, Host, T>
where
    Host: Runtime, T: StoreType
{
    fn log(&self, msg: String) {
        self.host.write_debug(msg.as_str())
    }

    fn has(&self, key: String) -> Result<bool> {
        if self.state.contains_key(&key) {
            return Ok(true);
        }

        if let Some(has) = self.saved_state.get(&key) {
            return Ok(*has);
        }

        match self.host.store_has(&str_to_path!(&key))? {
            Some(_) => Ok(true),
            None => Ok(false)
        }
    }

    fn get(&mut self, key: String) -> Result<Option<T>> {
        if let Some(val) = self.state.get(&key) {
            return Ok(val.clone());
        }

        let store_key = match self.saved_state.get(&key) {
            Some(false) => return Ok(None),
            Some(true) => [TMP_PREFIX, &key].concat(),
            None => key.clone(),
        };

        match self.host.store_read_all(&str_to_path!(&store_key)) {
            Ok(bytes) => {
                let val = T::from_vec(bytes)?;
                self.state.insert(key, Some(val.clone()));
                Ok(Some(val))
            }
            Err(RuntimeError::PathNotFound) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    fn set(&mut self, key: String, val: Option<T>) -> Result<()> {
        self.state.insert(key.clone(), val);
        self.modified_keys.insert(key);
        Ok(())
    }

    fn has_pending_changes(&self) -> bool {
        !self.modified_keys.is_empty()
    }

    fn commit(&mut self) -> Result<()> {
        let modified_keys: Vec<String> = self.modified_keys.drain().collect();
        for key in modified_keys {
            let val = self
                .state
                .remove(&key)
                .expect("Modified key must be in the pending state");

            let exists = match val {
                Some(val) => {
                    self.host.store_write_all(&str_to_path!([TMP_PREFIX, &key].concat().as_str()), &val.to_vec()?)?;
                    true
                }
                None => false,
            };
            self.saved_state.insert(key, exists);
        }
        Ok(())
    }

    fn rollback(&mut self) {
        for key in self.modified_keys.drain().into_iter() {
            self.state.remove(&key);
        }
    }

    fn clear(&mut self) {
        self.state.clear();
        self.saved_state.clear();
        self.modified_keys.clear();
        
        match self.host.store_delete(&str_to_path!(TMP_PREFIX)) {
            Ok(()) => {},
            Err(RuntimeError::PathNotFound) => {},
            Err(err) => panic!("Failed to clear kernel storage: {}", err)
        }
    }
}

#[cfg(test)]
mod test {
    use tezos_smart_rollup_mock::MockHost;
    use crate::{LayeredStore, StoreType, Result, kernel::KernelStore};

    #[derive(Clone)]
    pub struct EphemeralStoreType {
        pub value: i64
    }

    impl StoreType for EphemeralStoreType {
        fn from_vec(value: Vec<u8>) -> Result<Self> {
            Ok(Self {
                value: i64::from_be_bytes(value.as_slice().try_into().unwrap())
            })
        }

        fn to_vec(&self) -> Result<Vec<u8>> {
            Ok(self.value.to_be_bytes().to_vec())
        }
    }

    #[test]
    fn test_kernel_store() -> Result<()> {
        let mut host = MockHost::default();
        let mut store: KernelStore<MockHost, EphemeralStoreType> = KernelStore::new(&mut host);

        assert!(store.get("/test".into())?.is_none());

        store.set("/test".into(), Some(EphemeralStoreType { value: 42 }))?; // cached
        store.commit()?; // write to tmp folder
        store.persist()?; // move/delete permanently
        store.clear(); // clean up

        assert!(store.get("/test".into())?.is_some()); // cached again
        assert_eq!(42, store.get("/test".into())?.unwrap().value); // served from the cache

        Ok(())
    }
}
