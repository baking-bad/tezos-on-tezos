pub mod ephemeral;
pub mod head;
pub mod migrations;
pub mod types;
pub mod interpreter;
pub mod proto;

use crate::{
    context::types::ContextNode,
    Result,
};

pub trait Context {
    fn log(&self, msg: String);
    fn has(&self, key: String) -> Result<bool>;
    fn get(&mut self, key: String) -> Result<Option<ContextNode>>;
    fn set(&mut self, key: String, val: ContextNode) -> Result<()>;
    fn has_pending_changes(&self) -> bool;
    fn agg_pending_changes(&mut self) -> Vec<(String, Option<ContextNode>)>;
    fn save(&mut self, key: String, val: Option<ContextNode>) -> Result<Option<ContextNode>>;
    fn clear(&mut self);
}
