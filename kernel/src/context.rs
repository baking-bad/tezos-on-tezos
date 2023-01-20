use host::{
    path::RefPath,
    runtime::{Runtime, ValueType},
};
use std::collections::{HashMap, HashSet};
use context::{Result, GenericContext, ContextNode};

fn err_into(e: impl std::fmt::Debug) -> context::Error {
    context::Error::Internal(context::error::InternalError::new(
        context::error::InternalKind::Encoding,
        format!("PVM context error: {:?}", e)
    ))
}

pub struct PVMContext<Host>
where
    Host: Runtime,
{
    host: Host,
    state: HashMap<String, Option<ContextNode>>,
    modified_keys: HashSet<String>,
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
            modified_keys: HashSet::new(),
        }
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
        match self.state.contains_key(&key) {
            true => Ok(true),
            false => {
                let path = RefPath::assert_from(key.as_bytes());
                match self.host.store_has(&path) {
                    Ok(Some(ValueType::Value)) => Ok(true),
                    Err(err) => Err(err_into(err)),
                    _ => Ok(false),
                }
            }
        }
    }

    fn get(&mut self, key: String) -> Result<Option<ContextNode>> {
        match self.state.get(&key) {
            Some(cached_value) => Ok(cached_value.clone()),
            None => {
                let path = RefPath::assert_from(key.as_bytes());
                match self.host.store_has(&path) {
                    Ok(Some(ValueType::Value)) => {
                        // TODO: read loop for values > 2048
                        let stored_value = self.host.store_read(&path, 0, 512).map_err(err_into)?;
                        let value = ContextNode::from_vec(stored_value)?;
                        self.state.insert(key, Some(value.clone()));
                        Ok(Some(value))
                    }
                    Ok(Some(node_type)) => Err(err_into(node_type)),
                    Ok(None) => Ok(None),
                    Err(err) => Err(err_into(err)),
                }
            }
        }
    }

    fn set(&mut self, key: String, val: Option<ContextNode>) -> Result<()> {
        self.state.insert(key.clone(), val);
        self.modified_keys.insert(key);
        Ok(())
    }

    fn save(&mut self, key: String, val: Option<ContextNode>) -> Result<()> {
        let path = RefPath::assert_from(key.as_bytes());
        let res = match val {
            Some(val) => {
                let raw_value = val.to_vec()?;
                self.host.store_write(&path, raw_value.as_slice(), 0)
            },
            None => {
                self.host.store_delete(&path)
            }
        };
        res.map_err(err_into)
    }

    fn has_pending_changes(&self) -> bool {
        !self.modified_keys.is_empty()
    }

    fn agg_pending_changes(&mut self) -> Vec<(String, Option<ContextNode>)> {
        let mut changes: Vec<(String, Option<ContextNode>)> =
            Vec::with_capacity(self.modified_keys.len());
        for key in self.modified_keys.drain().into_iter() {
            let val = self.state
                .remove(&key)
                .expect("Modified key must be in the pending state");
            changes.push((key, val));
        }
        changes
    }

    fn clear(&mut self) {
        self.state.clear();
        self.modified_keys.clear();
    }
}

#[cfg(test)]
mod test {
    use mock_runtime::host::MockHost;
    use context::{GenericContext, ExecutorContext, Result};

    use crate::context::PVMContext;

    #[test]
    fn store_balance() -> Result<()> {
        let mut context = PVMContext::new(MockHost::default());

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
