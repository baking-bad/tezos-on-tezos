use std::collections::HashMap;

use crate::{LayeredStore, StoreType, Result};

pub struct EphemeralStore<T: StoreType> {
    state: HashMap<String, T>,
    pending_state: HashMap<String, Option<T>>,
    modified_keys: Vec<String>,
}

impl<T: StoreType> EphemeralStore<T> {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
            pending_state: HashMap::new(),
            modified_keys: Vec::new(),
        }
    }

    pub fn spawn(&self) -> Self {
        Self {
            state: self.state.clone(),
            pending_state: HashMap::new(),
            modified_keys: Vec::new(),
        }
    }

    pub fn pending_removed(&self, key: &String) -> bool {
        match self.pending_state.get(key) {
            Some(None) => true,
            _ => false,
        }
    }
}

impl<T: StoreType> LayeredStore<T> for EphemeralStore<T> {
    fn log(&self, msg: String) {
        eprintln!("[DEBUG] {}", msg);
    }

    fn has(&self, key: String) -> Result<bool> {
        match self.pending_state.get(&key) {
            Some(Some(_)) => Ok(true),
            Some(None) => Ok(false),
            None => Ok(self.state.contains_key(&key)),
        }
    }

    fn get(&mut self, key: String) -> Result<Option<T>> {
        // self.log(format!("get {}", &key));
        match self.pending_state.get(&key) {
            Some(cached_value) => Ok(cached_value.to_owned()),
            None => match self.state.get(&key) {
                Some(value) => {
                    self.pending_state.insert(key, Some(value.to_owned()));
                    Ok(Some(value.to_owned()))
                }
                None => Ok(None),
            },
        }
    }

    fn set(&mut self, key: String, val: Option<T>) -> Result<()> {
        // self.log(format!("set {} = {:?}", &key, &val));
        self.pending_state.insert(key.clone(), val);
        self.modified_keys.push(key);
        Ok(())
    }

    fn has_pending_changes(&self) -> bool {
        !self.modified_keys.is_empty()
    }

    fn commit(&mut self) -> Result<()> {
        while !self.modified_keys.is_empty() {
            let key = self.modified_keys.remove(0);
            let val = self
                .pending_state
                .get(&key)
                .expect("Modified key must be in the pending state");

            match val {
                Some(val) => self.state.insert(key, val.clone()),
                None => self.state.remove(&key),
            };
        }
        Ok(())
    }

    fn rollback(&mut self) {
        for key in self.modified_keys.drain(..) {
            self.pending_state.remove(&key);
        }
    }

    fn clear(&mut self) {
        self.pending_state.clear();
        self.modified_keys.clear();
    }
}

#[cfg(test)]
mod test {
    use crate::{ephemeral::EphemeralStore, LayeredStore, StoreType, Result};
    
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
    fn test_mock_store() -> Result<()> {
        let mut store: EphemeralStore<EphemeralStoreType> = EphemeralStore::new();

        assert!(store.get("/test".into())?.is_none());

        store.set("/test".into(), Some(EphemeralStoreType { value: 42 }))?; // cached
        store.commit()?; // write to tmp folder
        store.clear(); // clean up

        assert!(store.get("/test".into())?.is_some()); // cached again
        assert_eq!(42, store.get("/test".into())?.unwrap().value); // served from the cache

        Ok(())
    }
}
