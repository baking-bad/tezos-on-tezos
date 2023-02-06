use std::collections::HashMap;

use crate::{ContextNode, GenericContext, Result};

pub struct EphemeralContext {
    state: HashMap<String, ContextNode>,
    pending_state: HashMap<String, Option<ContextNode>>,
    modified_keys: Vec<String>,
}

impl EphemeralContext {
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

impl GenericContext for EphemeralContext {
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

    fn get(&mut self, key: String) -> Result<Option<ContextNode>> {
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

    fn set(&mut self, key: String, val: Option<ContextNode>) -> Result<()> {
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
    use crate::{EphemeralContext, ExecutorContext, GenericContext, Result};
    use tezos_core::types::mutez::Mutez;

    #[test]
    fn store_balance() -> Result<()> {
        let mut context = EphemeralContext::new();

        let address = "tz1Mj7RzPmMAqDUNFBn5t5VbXmWW4cSUAdtT";
        let balance: Mutez = 1000u32.into();

        assert!(context.get_balance(&address)?.is_none()); // both host and cache accessed

        context.set_balance(&address, balance.clone())?; // cached
        context.commit()?; // save
        context.clear(); // cache cleared

        assert!(context.get_balance(&address)?.is_some()); // cached
        assert_eq!(
            context
                .get_balance(&address)?
                .expect("Balance must not be null"),
            balance
        ); // served from the cache

        Ok(())
    }
}
