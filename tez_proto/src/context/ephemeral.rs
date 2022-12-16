use std::collections::HashMap;

use crate::context::Context;
use crate::context::node::{ContextNode, NodeType};
use crate::error::Result;

pub struct EphemeralContext {
    state: HashMap<String, ContextNode>,
    pending_state: HashMap<String, ContextNode>,
    modified_keys: Vec<String>
}

impl EphemeralContext {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
            pending_state: HashMap::new(),
            modified_keys: Vec::new()
        }
    }
}

impl Context for EphemeralContext {
    fn has(&self, key: String) -> Result<bool> {
        match self.pending_state.contains_key(&key) {
            true => Ok(true),
            false => Ok(self.state.contains_key(&key))
        }
    }

    fn get<V: NodeType>(&mut self, key: String, _max_bytes: usize) -> Result<Option<V>> {
        match self.pending_state.get(&key) {
            Some(cached_value) => Ok(Some(V::unwrap(cached_value))),
            None => match self.state.get(&key) {
                Some(value) => {
                    self.pending_state.insert(key, value.clone());
                    Ok(Some(V::unwrap(value)))
                },
                None => Ok(None)
            }
        }
    }

    fn set<V: NodeType>(&mut self, key: String, val: &V) -> Result<()> {
        self.pending_state.insert(key.clone(), V::wrap(val));
        self.modified_keys.push(key);
        Ok(())
    }

    fn has_pending_changes(&self) -> bool {
        !self.modified_keys.is_empty()
    }

    fn commit(&mut self) -> Result<()> {
        for key in self.modified_keys.iter() {
            let val = self.pending_state.get(key).expect("Expected value");
            self.state.insert(key.clone(), val.clone());
        }
        self.modified_keys.clear();
        Ok(())
    }

    fn clear(&mut self) {
        self.pending_state.clear();
    }

    fn rollback(&mut self) {
        for key in self.modified_keys.iter() {
            self.pending_state.remove(key);
        }
        self.modified_keys.clear();
    }
}


#[cfg(test)]
mod test {
    use crate::{
        error::Result,
        context::Context,
        context::ephemeral::EphemeralContext
    };

    #[test]
    fn store_balance() -> Result<()> {
        let mut context = EphemeralContext::new();

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