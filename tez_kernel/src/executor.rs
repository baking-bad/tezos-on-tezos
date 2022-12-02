pub mod reveal;
pub mod transaction;

use host::runtime::Runtime;
use tezos_operation::{
    operations::{SignedOperation, OperationContent}
};
use tezos_rpc::models::operation::{
    Operation as OperationWithResult
};

use crate::error::Result;
use crate::context::EphemeralContext;
use crate::executor::{
    reveal::execute_reveal, 
    transaction::execute_transaction
};

pub fn execute_operation(host: &mut impl Runtime, context: &mut EphemeralContext, opg: &SignedOperation) -> Result<OperationWithResult> {
    if context.has_pending_changes() {
        panic!("Cannot proceed with uncommited changes");
    }

    let mut contents = Vec::new();
    for content in opg.contents.iter() {
        let content_with_result = match content {
            OperationContent::Reveal(reveal) => execute_reveal(host, context, reveal),
            OperationContent::Transaction(transaction) => execute_transaction(host, context, transaction),
            OperationContent::Origination(_origination) => todo!("Implement origination"),
            _ => panic!("Operation type not supported")
        };

        match content_with_result {
            Err(error) => return Err(error),
            Ok(value) => contents.push(value)
        }
    }

    let operation_with_metadata = OperationWithResult {
        protocol: None,
        chain_id: None,
        hash: None,  // blake2b of forged bytes + dummy sig
        branch: opg.branch.clone(),
        signature: Some(opg.signature.clone()),
        contents
    };
    Ok(operation_with_metadata)
}