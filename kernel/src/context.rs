use std::collections::{HashMap, HashSet};
use tezos_ctx::{ContextNode, GenericContext, Result};
use tezos_smart_rollup_host::runtime::Runtime;

use crate::store::{store_delete, store_has, store_move, store_read, store_write};

const TMP_PREFIX: &str = "/tmp";

pub struct PVMContext<Host>
where
    Host: Runtime,
{
    host: Host,
    state: HashMap<String, Option<ContextNode>>,
    modified_keys: HashSet<String>,
    saved_state: HashMap<String, bool>,
}

impl<Host> AsMut<Host> for PVMContext<Host>
where
    Host: Runtime,
{
    fn as_mut(&mut self) -> &mut Host {
        &mut self.host
    }
}

impl<Host> PVMContext<Host>
where
    Host: Runtime,
{
    pub fn new(host: Host) -> Self {
        PVMContext {
            host,
            state: HashMap::new(),
            saved_state: HashMap::new(),
            modified_keys: HashSet::new(),
        }
    }
}

impl<Host: Runtime> PVMContext<Host> {
    pub fn persist(&mut self) -> Result<()> {
        for (key, exists) in self.saved_state.drain() {
            if exists {
                store_move(&mut self.host, [TMP_PREFIX, &key].concat().as_str(), &key)?;
            } else {
                store_delete(&mut self.host, &key)?;
            }
        }
        Ok(())
    }
}

impl<Host> GenericContext for PVMContext<Host>
where
    Host: Runtime,
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

        store_has(&self.host, &key)
    }

    fn get(&mut self, key: String) -> Result<Option<ContextNode>> {
        if let Some(val) = self.state.get(&key) {
            return Ok(val.clone());
        }

        let store_key = match self.saved_state.get(&key) {
            Some(false) => return Ok(None),
            Some(true) => [TMP_PREFIX, &key].concat(),
            None => key.clone(),
        };

        match store_read(&self.host, &store_key) {
            Ok(Some(bytes)) => {
                let val = ContextNode::from_vec(bytes)?;
                self.state.insert(key, Some(val.clone()));
                Ok(Some(val))
            }
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn set(&mut self, key: String, val: Option<ContextNode>) -> Result<()> {
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
                    store_write(
                        &mut self.host,
                        [TMP_PREFIX, &key].concat().as_str(),
                        val.to_vec()?,
                    )?;
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
        store_delete(&mut self.host, TMP_PREFIX).expect("Failed to remove tmp files")
    }
}

#[cfg(test)]
mod test {
    use mock_runtime::MockHost;
    use tezos_core::types::mutez::Mutez;
    use tezos_ctx::{ExecutorContext, GenericContext, Result};

    use crate::context::PVMContext;

    #[test]
    fn store_balance() -> Result<()> {
        let mut context = PVMContext::new(MockHost::default());

        let address = "tz1Mj7RzPmMAqDUNFBn5t5VbXmWW4cSUAdtT";
        let balance: Mutez = 1000u32.into();

        assert!(context.get_balance(&address)?.is_none()); // both host and cache accessed

        context.set_balance(&address, balance.clone())?; // cached
        context.commit()?; // write to tmp folder
        context.persist()?; // move/delete permanently
        context.clear(); // clean up

        assert!(context.get_balance(&address)?.is_some()); // cached again
        assert_eq!(
            context
                .get_balance(&address)?
                .expect("Balance must not be null"),
            balance
        ); // served from the cache

        Ok(())
    }
}
