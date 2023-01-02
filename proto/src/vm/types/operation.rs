use std::fmt::Display;
use tezos_operation::operations::OperationContent;

use crate::{
    vm::types::{OperationItem}
};

impl OperationItem {
    pub fn into_content(self) -> OperationContent {
        self.0
    }
}

impl Display for OperationItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Operation")
    }
}