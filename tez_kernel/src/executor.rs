pub mod reveal;
pub mod transaction;
pub mod runtime_error;
pub mod balance_update;

use host::runtime::Runtime;
use tezos_core::types::mutez::Mutez;
use tezos_operation::{
    operations::{SignedOperation, OperationContent}
};
use tezos_rpc::models::operation::{
    Operation as OperationReceipt,
    OperationContent as OperationContentReceipt, operation_result::OperationResultStatus
};

use crate::execution_error;
use crate::error::Result;
use crate::context::EphemeralContext;
use crate::executor::{
    reveal::{execute_reveal, skip_reveal}, 
    transaction::{execute_transaction, skip_transaction}
};
use crate::validator::validate_operation;

fn update_status(receipt: &mut OperationContentReceipt, status: OperationResultStatus) -> Result<()> {
    match receipt {
        OperationContentReceipt::Reveal(reveal) => {
            if let Some(metadata) = reveal.metadata.as_mut() {
                metadata.operation_result.status = status;
            }
        },
        OperationContentReceipt::Transaction(transaction) => {
            if let Some(metadata) = transaction.metadata.as_mut() {
                metadata.operation_result.status = status;
            }
        },
        OperationContentReceipt::Origination(_origination) => todo!("Implement for origination"),
        _ => return execution_error!("Operation kind not supported: {:?}", receipt)
    };
    Ok(())
}

fn get_status(receipt: &OperationContentReceipt) -> Result<OperationResultStatus> {
    match receipt {
        OperationContentReceipt::Reveal(reveal) => {
            if let Some(metadata) = reveal.metadata.as_ref() {
                return Ok(metadata.operation_result.status.to_owned());
            }
        },
        OperationContentReceipt::Transaction(transaction) => {
            if let Some(metadata) = transaction.metadata.as_ref() {
                return Ok(metadata.operation_result.status.to_owned());
            }
        },
        OperationContentReceipt::Origination(_origination) => todo!("Implement for origination"),
        _ => panic!("Operation kind not supported: {:?}", receipt)
    }
    execution_error!("Operation metadata is missing")
}

pub fn execute_operation(host: &mut impl Runtime, context: &mut EphemeralContext, opg: &SignedOperation) -> Result<OperationReceipt> {
    if context.has_pending_changes() {
        return execution_error!("Cannot proceed with uncommited changes");
    }

    let plan = validate_operation(host, context, opg)?;
    let mut contents = Vec::new();
    let mut failed_idx: Option<usize> = None;

    // reserve funds for execution expenses
    context.set_balance(&plan.source, &(plan.balance - plan.total_fees))?;

    for (i, content) in opg.contents.iter().enumerate() {
        let receipt = match content {
            OperationContent::Reveal(reveal) => {
                let reveal_receipt = if failed_idx.is_none() {
                    execute_reveal(host, context, reveal)?
                } else {
                    skip_reveal(reveal.to_owned())
                };
                OperationContentReceipt::Reveal(reveal_receipt)
            },
            OperationContent::Transaction(transaction) => {
                let transaction_receipt = if failed_idx.is_none() {
                    execute_transaction(host, context, transaction)?
                } else {
                    skip_transaction(transaction.to_owned())
                };
                OperationContentReceipt::Transaction(transaction_receipt)
            },
            OperationContent::Origination(_origination) => todo!("Implement origination"),
            content => return execution_error!("Operation kind not supported: {:?}", content)
        };

        if get_status(&receipt)? == OperationResultStatus::Failed {
            failed_idx = Some(i);
            context.rollback();
        }

        contents.push(receipt);
    }

    if let Some(stop) = failed_idx {
        for i in 0..stop {
            update_status(&mut contents[i], OperationResultStatus::Backtracked)?;
        }
        let total_fees: Mutez = opg.contents[0..=stop].iter().map(|c| c.fee()).sum();
        context.set_balance(&plan.source, &(plan.balance - total_fees))?;
    } else {
        context.set_counter(&plan.source, &plan.last_counter)?;
    }

    context.commit(host)?;

    Ok(OperationReceipt {
        protocol: None,
        chain_id: None,
        hash: None,  // blake2b of forged bytes + dummy sig
        branch: opg.branch.clone(),
        signature: Some(opg.signature.clone()),
        contents
    })
}