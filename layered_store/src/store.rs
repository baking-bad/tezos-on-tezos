// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use crate::{Error, Result};
use std::{
    any::Any,
    collections::{HashMap, HashSet},
};

pub type DynStoreType = Box<dyn Any + Send>;
pub type StoreTypeSer = Box<dyn Fn(&DynStoreType) -> Result<Vec<u8>> + Send>;

pub trait StoreType: Sized + Clone + Send + 'static {
    fn from_bytes(bytes: &[u8]) -> Result<Self>;
    fn to_bytes(&self) -> Result<Vec<u8>>;

    fn downcast_ref(value: &DynStoreType) -> Result<&Self> {
        let res = value
            .downcast_ref::<Self>()
            .ok_or(Error::DowncastingError)?;
        Ok(res)
    }

    fn serialize(value: &DynStoreType) -> Result<Vec<u8>> {
        let val = Self::downcast_ref(value)?;
        val.to_bytes()
    }
}

pub trait StoreBackend {
    fn default() -> Self;
    fn log(&self, msg: &str);
    fn has(&self, key: &str) -> Result<bool>;
    fn read(&self, key: &str) -> Result<Option<Vec<u8>>>;
    fn write(&mut self, key: &str, val: &[u8]) -> Result<()>;
    fn delete(&mut self, key: &str) -> Result<()>;
    fn clear(&mut self);
}

pub struct LayeredStore<Backend: StoreBackend> {
    backend: Backend,
    pending_state: HashMap<String, Option<(DynStoreType, StoreTypeSer)>>,
    modified_keys: HashSet<String>,
}

impl<Backend: StoreBackend> LayeredStore<Backend> {
    pub fn new(backend: Backend) -> Self {
        Self {
            backend,
            pending_state: HashMap::new(),
            modified_keys: HashSet::new(),
        }
    }

    pub fn default() -> Self {
        Self::new(Backend::default())
    }

    pub fn log(&self, msg: String) {
        self.backend.log(&msg);
    }

    pub fn has(&self, key: String) -> Result<bool> {
        if self.pending_state.contains_key(&key) {
            return Ok(true);
        }
        self.backend.has(&key)
    }

    pub fn has_pending_changes(&self) -> bool {
        !self.modified_keys.is_empty()
    }

    pub fn pending_removed(&self, key: &String) -> bool {
        match self.pending_state.get(key) {
            Some(None) => true,
            _ => false,
        }
    }

    pub fn get<T: StoreType>(&mut self, key: String) -> Result<Option<T>> {
        match self.pending_state.get(&key) {
            Some(Some((dyn_value, _))) => Ok(Some(T::downcast_ref(dyn_value)?.clone())),
            Some(None) => Ok(None),
            None => match self.backend.read(&key)? {
                Some(bytes) => {
                    let value = T::from_bytes(&bytes)?;
                    self.pending_state
                        .insert(key, Some((Box::new(value.clone()), Box::new(T::serialize))));
                    Ok(Some(value))
                }
                None => Ok(None),
            },
        }
    }

    pub fn set<T: StoreType>(&mut self, key: String, val: Option<T>) -> Result<()> {
        match val {
            Some(value) => self
                .pending_state
                .insert(key.clone(), Some((Box::new(value), Box::new(T::serialize)))),
            None => self.pending_state.insert(key.clone(), None),
        };
        self.modified_keys.insert(key);
        Ok(())
    }

    pub fn commit(&mut self) -> Result<()> {
        let modified_keys: Vec<String> = self.modified_keys.drain().collect();
        for key in modified_keys {
            let val = self
                .pending_state
                .get(&key)
                .expect("Modified key must be in the pending state");

            match val {
                Some((dyn_value, ser)) => {
                    let bytes = ser(dyn_value)?;
                    self.backend.write(&key, bytes.as_slice())?;
                }
                None => {
                    self.backend.delete(&key)?;
                }
            };
        }
        Ok(())
    }

    pub fn rollback(&mut self) {
        for key in self.modified_keys.drain().into_iter() {
            self.pending_state.remove(&key);
        }
    }

    pub fn clear(&mut self) {
        self.pending_state.clear();
        self.modified_keys.clear();
        self.backend.clear();
    }
}

impl<Backend: StoreBackend> AsMut<Backend> for LayeredStore<Backend> {
    fn as_mut(&mut self) -> &mut Backend {
        &mut self.backend
    }
}

impl<Backend: StoreBackend> AsRef<Backend> for LayeredStore<Backend> {
    fn as_ref(&self) -> &Backend {
        &self.backend
    }
}
