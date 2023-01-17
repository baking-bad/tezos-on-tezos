use tezos_core::types::encoded::OperationHash;
use tezos_operation::operations::SignedOperation;

use crate::{
    assert_no_pending_changes,
    context::Context,
    validator::{validate_operation, ManagerOperation},
    Result,
};

pub fn validate_batch(
    context: &mut impl Context,
    batch_payload: Vec<(OperationHash, SignedOperation)>,
    atomic: bool,
) -> Result<Vec<ManagerOperation>> {
    assert_no_pending_changes!(context);

    let mut operations: Vec<ManagerOperation> = Vec::with_capacity(batch_payload.len());

    for (hash, opg) in batch_payload.into_iter() {
        match validate_operation(context, opg, hash) {
            Ok(op) => {
                let balance = context.get_balance(&op.source)?.unwrap();
                context.set_balance(&op.source, &(balance - op.total_spent))?;
                context.set_counter(&op.source, &op.last_counter)?;
                operations.push(op);
            }
            Err(err) => {
                context.log(err.to_string());
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
