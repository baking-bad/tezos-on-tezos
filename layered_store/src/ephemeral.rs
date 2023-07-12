// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use crate::{store::StoreBackend, LayeredStore, Result};

pub struct EphemeralBackend {
    pub(super) state: HashMap<String, Vec<u8>>,
}

impl EphemeralBackend {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    pub fn spawn(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

impl StoreBackend for EphemeralBackend {
    fn default() -> Self {
        Self::new()
    }

    fn log(&self, msg: &str) {
        eprintln!("[DEBUG] {}", msg);
    }

    fn has(&self, key: &str) -> Result<bool> {
        Ok(self.state.contains_key(key))
    }

    fn read(&self, key: &str) -> Result<Option<Vec<u8>>> {
        Ok(self.state.get(key).cloned())
    }

    fn write(&mut self, key: &str, val: &[u8]) -> Result<()> {
        self.state.insert(key.into(), val.to_vec());
        Ok(())
    }

    fn delete(&mut self, key: &str) -> Result<()> {
        self.state.remove(key);
        Ok(())
    }

    fn clear(&mut self) {}
}

pub trait EphemeralCopy {
    fn spawn(&self) -> Self;
}

impl EphemeralCopy for LayeredStore<EphemeralBackend> {
    fn spawn(&self) -> Self {
        Self::new(self.as_ref().spawn())
    }
}

#[cfg(test)]
mod test {
    use crate::{ephemeral::EphemeralBackend, LayeredStore, Result, StoreType};

    #[derive(Clone, Debug)]
    pub struct TestType {
        pub value: i32,
    }

    impl StoreType for TestType {
        fn from_bytes(bytes: &[u8]) -> Result<Self> {
            let value = i32::from_be_bytes(bytes.try_into().unwrap());
            Ok(Self { value })
        }

        fn to_bytes(&self) -> Result<Vec<u8>> {
            Ok(self.value.to_be_bytes().to_vec())
        }
    }

    #[test]
    fn test_mock_store() -> Result<()> {
        let mut store = LayeredStore::<EphemeralBackend>::default();

        assert!(!store.has("/test".into())?);

        store.set("/test".into(), Some(TestType { value: 42 }))?; // cached
        store.commit()?; // write to tmp folder
        store.clear(); // clean up

        assert!(store.get::<TestType>("/test".into())?.is_some()); // cached again
        assert_eq!(42, store.get::<TestType>("/test".into())?.unwrap().value); // served from the cache

        Ok(())
    }
}
