use tezos_core::types::encoded::{Encoded, OperationHash};
use tezos_operation::operations::SignedOperation;

use crate::{
    validator::operation::{validate_operation, ValidOperation, ValidatedOperation},
    context::TezosContext,
    Result,
};

pub fn validate_batch(
    context: &mut impl TezosContext,
    batch_payload: Vec<(OperationHash, SignedOperation)>,
    atomic: bool,
) -> Result<Vec<ValidOperation>> {
    context.check_no_pending_changes()?;

    let mut operations: Vec<ValidOperation> = Vec::with_capacity(batch_payload.len());

    for (hash, opg) in batch_payload.into_iter() {
        match validate_operation(context, opg, hash.clone(), false) {
            Ok(ValidatedOperation::Valid(op)) => {
                let balance = context.get_balance(&op.source.value())?.unwrap();
                context.set_balance(&op.source.value(), balance - op.total_spent)?;
                context.set_counter(&op.source.value(), op.last_counter.clone())?;
                operations.push(op);
            }
            Ok(ValidatedOperation::Invalid(op)) => {
                context.log(format!("Invalid operation: {:#?}", op));
                if atomic {
                    context.rollback();
                    return Ok(vec![]);
                }
            }
            Err(err) => {
                context.log(format!(
                    "Validator error: {}\n{}",
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
