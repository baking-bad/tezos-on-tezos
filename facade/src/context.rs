use log::debug;
use std::collections::HashMap;
use context::{GenericContext, ContextNode};

pub struct RPCContext {
    tmp_state: HashMap<String, Option<ContextNode>>,
    level: i32
}

impl RPCContext {
    
}

impl GenericContext for RPCContext {
    fn log(&self, msg: String) {
        debug!("{}", msg)
    }

    fn has(&self, key: String) -> context::Result<bool> {
        match self.tmp_state.get(&key) {
            Some(Some(_)) => return Ok(true),
            Some(None) => return Ok(false),
            None => {}
        };
        Ok(false)
    }

    fn get(&mut self, key: String) -> context::Result<Option<ContextNode>> {
        Ok(None)
    }

    fn set(&mut self, key: String, val: Option<ContextNode>) -> context::Result<()> {
        self.tmp_state.insert(key, val);
        Ok(())
    }
}