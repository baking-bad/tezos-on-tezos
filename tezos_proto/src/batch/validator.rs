// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use tezos_core::types::encoded::{Encoded, OperationHash};
use tezos_operation::operations::SignedOperation;

use crate::{
    context::TezosContext,
    operations::validator::{validate_operation, ValidOperation, ValidatedOperation},
    Error, Result,
};

use super::payload::BatchPayload;

macro_rules! assert_attr_eq {
    ($expected: expr, $actual: expr) => {
        if $actual != $expected {
            return Err(Error::InvalidBatchHeader {
                reason: format!(
                    "{}: expected {:?}, got {:?}",
                    stringify!($actual),
                    $expected,
                    $actual
                ),
            });
        }
    };
}

pub fn validate_explicit_batch(
    context: &mut impl TezosContext,
    batch: BatchPayload,
    dry_run: bool,
) -> Result<Vec<ValidOperation>> {
    context.check_no_pending_changes()?;

    let prev_head = context.get_head()?;

    assert_attr_eq!(prev_head.level + 1, batch.header.level);
    assert_attr_eq!(prev_head.hash, batch.header.predecessor);

    if batch.header.timestamp < prev_head.timestamp + 1 {
        return Err(Error::InvalidBatchHeader {
            reason: format!(
                "batch.header.timestamp: expected at least {}, got {}",
                prev_head.timestamp + 1,
                batch.header.timestamp
            ),
        });
    }

    assert_attr_eq!(
        batch.operation_list_list_hash()?,
        batch.header.operations_hash
    );

    if !dry_run {
        // TODO: check signature against whitelisted sequencer account
    }

    let mut valid_operations: Vec<ValidOperation> = Vec::with_capacity(batch.operations.len());

    for (hash, opg) in batch.operations.into_iter() {
        match validate_operation(context, opg, hash.clone(), dry_run) {
            Ok(ValidatedOperation::Valid(op)) => {
                let balance = context.get_balance(&op.source.value())?.unwrap();
                context.set_balance(&op.source.value(), balance - op.total_spent)?;
                context.set_counter(&op.source.value(), op.last_counter.clone())?;
                valid_operations.push(op);
            }
            Ok(ValidatedOperation::Invalid(errors)) => {
                context.log(format!(
                    "Invalid batch operation {}\n{:#?}",
                    hash.value(),
                    errors
                ));
                context.rollback();
                return Err(Error::InvalidBatchOperation {
                    reason: format!("{:#?}", errors),
                });
            }
            Err(err) => {
                context.log(format!(
                    "Batch validator error at {}\n{}",
                    hash.value(),
                    err.format()
                ));
                context.rollback();
                return Err(err);
            }
        }
    }

    context.rollback();

    Ok(valid_operations)
}

pub fn validate_implicit_batch(
    context: &mut impl TezosContext,
    operations: Vec<(OperationHash, SignedOperation)>,
    dry_run: bool,
) -> Result<Vec<ValidOperation>> {
    context.check_no_pending_changes()?;

    let mut valid_operations: Vec<ValidOperation> = Vec::with_capacity(operations.len());

    for (hash, opg) in operations.into_iter() {
        match validate_operation(context, opg, hash.clone(), dry_run) {
            Ok(ValidatedOperation::Valid(op)) => {
                let balance = context.get_balance(&op.source.value())?.unwrap();
                context.set_balance(&op.source.value(), balance - op.total_spent)?;
                context.set_counter(&op.source.value(), op.last_counter.clone())?;
                valid_operations.push(op);
            }
            Ok(ValidatedOperation::Invalid(errors)) => {
                context.log(format!("Invalid operation: {:#?}", errors));
            }
            Err(err) => {
                context.log(format!(
                    "Validator error: {}\n{}",
                    hash.value(),
                    err.format()
                ));
            }
        }
    }

    context.rollback();

    Ok(valid_operations)
}
