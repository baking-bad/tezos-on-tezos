use std::collections::HashMap;
use vm::interpreter::InterpreterContext;

use crate::{
    context::{types::{ContextNode}, Context},
    Result
};
pub struct EphemeralContext {
    state: HashMap<String, ContextNode>,
    pending_state: HashMap<String, ContextNode>,
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
}

impl Context for EphemeralContext {
    fn log(&self, msg: String) {
        eprintln!("[DEBUG] {}", msg);
    }

    fn has(&self, key: String) -> Result<bool> {
        match self.pending_state.contains_key(&key) {
            true => Ok(true),
            false => Ok(self.state.contains_key(&key)),
        }
    }

    fn get(&mut self, key: String) -> Result<Option<ContextNode>> {
        match self.pending_state.get(&key) {
            Some(cached_value) => Ok(Some(cached_value.to_owned())),
            None => match self.state.get(&key) {
                Some(value) => {
                    self.pending_state.insert(key, value.to_owned());
                    Ok(Some(value.to_owned()))
                }
                None => Ok(None),
            },
        }
    }

    fn set(&mut self, key: String, val: ContextNode) -> Result<()> {
        self.pending_state.insert(key.clone(), val);
        self.modified_keys.push(key);
        Ok(())
    }

    fn has_pending_changes(&self) -> bool {
        !self.modified_keys.is_empty()
    }

    fn agg_pending_changes(&mut self) -> Vec<(String, Option<ContextNode>)> {
        let mut changes: Vec<(String, Option<ContextNode>)> = Vec::with_capacity(self.modified_keys.len());
        while !self.modified_keys.is_empty() {
            let key = self.modified_keys.remove(0);
            let val = self.pending_state.remove(&key).expect("Modified key must be in the pending state");
            changes.push((key, Some(val)));
        }
        changes
    }

    fn save(&mut self, key: String, val: Option<ContextNode>) -> Result<Option<ContextNode>> {
        let old = match val {
            Some(val) => self.state.insert(key, val.clone()),
            None => self.state.remove(&key)
        };
        Ok(old)
    }

    fn clear(&mut self) {
        self.pending_state.clear();
        self.modified_keys.clear();
    }
}

#[cfg(test)]
mod test {
    use crate::{
        context::ephemeral::EphemeralContext,
        context::Context,
        context::proto::ProtoContext,
        Result
    };

    #[test]
    fn store_balance() -> Result<()> {
        let mut context = EphemeralContext::new();

        let address = "tz1Mj7RzPmMAqDUNFBn5t5VbXmWW4cSUAdtT";
        let balance = 1000u32.into();

        assert!(context.get_balance(&address)?.is_none()); // both host and cache accessed

        context.set_balance(&address, &balance)?; // cached
        context.commit()?; // sent to the host
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
