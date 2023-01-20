use context::{ExecutorContext, GenericContext};
use tezos_core::types::encoded::{Encoded, OperationHash};
use tezos_operation::operations::SignedOperation;

use crate::{
    validator::operation::{validate_operation, ManagerOperation},
    Result,
};

pub fn validate_batch(
    context: &mut (impl GenericContext + ExecutorContext),
    batch_payload: Vec<(OperationHash, SignedOperation)>,
    atomic: bool,
) -> Result<Vec<ManagerOperation>> {
    context.check_no_pending_changes()?;

    let mut operations: Vec<ManagerOperation> = Vec::with_capacity(batch_payload.len());

    for (hash, opg) in batch_payload.into_iter() {
        match validate_operation(context, opg, hash.clone()) {
            Ok(op) => {
                let balance = context.get_balance(&op.source.value())?.unwrap();
                context.set_balance(&op.source.value(), &(balance - op.total_spent))?;
                context.set_counter(&op.source.value(), &op.last_counter)?;
                operations.push(op);
            }
            Err(err) => {
                context.log(format!(
                    "Invalid operation: {}\n{}",
                    hash.value(),
                    err.format()
                ));
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
