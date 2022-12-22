pub mod operation;

use tezos_operation::operations::SignedOperation;
use tezos_core::types::{
    encoded::{ImplicitAddress, OperationHash},
    mutez::Mutez,
    number::Nat
};

use crate::{
    error::Result,
    context::{Context, head::Head},
    validator::operation::validate_operation,
    producer::types::BatchHeader,
    validation_error
};

pub struct ManagerOperation {
    pub hash: OperationHash,
    pub origin: SignedOperation,
    pub source: ImplicitAddress,
    pub total_fees: Mutez,
    pub total_spent: Mutez,
    pub last_counter: Nat
}

pub fn validate_header(head: &Head, header: &BatchHeader) -> Result<()> {
    if header.predecessor != head.hash {
        return validation_error!("Invalid predecessor {:?} (expected {:?})", header.predecessor, head.hash);
    }
    
    if header.level != head.level + 1 {
        return validation_error!("Invalid level {} (expected {})", header.level, head.level + 1);
    }

    if header.timestamp <= head.timestamp {
        return validation_error!("Invalid timestamp {} (expected at least {})", header.timestamp, head.timestamp + 1);
    }

    // TODO: validate operation/payload

    Ok(())
}

pub fn validate_batch(context: &mut impl Context, batch_payload: Vec<(OperationHash, SignedOperation)>, atomic: bool) -> Result<Vec<ManagerOperation>> {
    if context.has_pending_changes() {
        return validation_error!("Cannot proceed with uncommitted state changes");
    }
    
    let mut operations: Vec<ManagerOperation> = Vec::with_capacity(batch_payload.len());
    
    for (hash, opg) in batch_payload.into_iter() {
        match validate_operation(context, opg, hash) {
            Ok(op) => {
                let balance = context.get_balance(&op.source)?.expect("Missing balance");
                context.set_balance(&op.source, &(balance - op.total_spent))?;
                context.set_counter(&op.source, &op.last_counter)?;
                operations.push(op);
            },
            Err(err) => {
                // TODO: context.error_log(err)
                if atomic {
                    context.rollback();
                    return Err(err); 
                }
            }
        }
    }

    context.rollback();
    Ok(operations)
}

