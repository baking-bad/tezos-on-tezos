pub mod reveal;
pub mod transaction;
pub mod origination;
pub mod balance_update;

use tezos_core::types::{
    mutez::Mutez,
    encoded::{ProtocolHash, ChainId, Encoded}
};
use tezos_operation::operations::OperationContent;
use tezos_rpc::models::operation::{
    Operation as OperationReceipt,
    OperationContent as OperationContentReceipt,
    operation_result::OperationResultStatus
};

use crate::{
    assert_no_pending_changes,
    error::{Error, Result},
    context::Context,
    executor::{
        reveal::{execute_reveal, skip_reveal}, 
        transaction::{execute_transaction, skip_transaction},
        origination::{execute_origination, skip_origination}
    },
    validator::ManagerOperation,
    constants::{CHAIN_ID, PROTOCOL}
};

macro_rules! set_status {
    ($receipt: expr, $status: expr) => {
        if let Some(metadata) = $receipt.metadata.as_mut() {
            metadata.operation_result.status = $status;
            return Ok(())
        }
    };
}

macro_rules! get_status {
    ($receipt: expr) => {
        if let Some(metadata) = $receipt.metadata.as_ref() {
            return Ok(metadata.operation_result.status.clone());
        }
    };
}

fn update_status(receipt: &mut OperationContentReceipt, status: OperationResultStatus) -> Result<()> {
    match receipt {
        OperationContentReceipt::Reveal(reveal) => set_status!(reveal, status),
        OperationContentReceipt::Transaction(transaction) => set_status!(transaction, status),
        OperationContentReceipt::Origination(origination) => set_status!(origination, status),
        _ => return Err(Error::OperationKindUnsupported)
    };
    panic!("Operation metadata is missing: {:?}", receipt)
}

fn get_status(receipt: &OperationContentReceipt) -> Result<OperationResultStatus> {
    match receipt {
        OperationContentReceipt::Reveal(reveal) => get_status!(reveal),
        OperationContentReceipt::Transaction(transaction) => get_status!(transaction),
        OperationContentReceipt::Origination(origination) => get_status!(origination),
        _ => return Err(Error::OperationKindUnsupported)
    }
    panic!("Operation metadata is missing: {:?}", receipt)
}

pub fn execute_operation(context: &mut impl Context, opg: &ManagerOperation) -> Result<OperationReceipt> {
    assert_no_pending_changes!(context);

    let initial_balance = context.get_balance(&opg.source)?.expect("Balance not initialized");
    let mut contents = Vec::new();
    let mut failed_idx: Option<usize> = None;
    let mut origination_index: i32 = 0;

    // reserve funds for execution expenses
    context.set_balance(&opg.source, &(initial_balance - opg.total_fees))?;

    for (i, content) in opg.origin.contents.iter().enumerate() {
        let receipt = match content {
            OperationContent::Reveal(reveal) => {
                let reveal_receipt = if failed_idx.is_none() {
                    execute_reveal(context, reveal)?
                } else {
                    skip_reveal(reveal.clone())
                };
                OperationContentReceipt::Reveal(reveal_receipt)
            },
            OperationContent::Transaction(transaction) => {
                let transaction_receipt = if failed_idx.is_none() {
                    execute_transaction(context, transaction)?
                } else {
                    skip_transaction(transaction.clone())
                };
                OperationContentReceipt::Transaction(transaction_receipt)
            },
            OperationContent::Origination(origination) => {
                let origination_receipt = if failed_idx.is_none() {
                    execute_origination(context, origination, &opg.hash, &mut origination_index)?
                } else {
                    skip_origination(origination.clone())
                };
                OperationContentReceipt::Origination(origination_receipt)
            }
            _ => return Err(Error::OperationKindUnsupported)
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
        let total_fees: Mutez = opg.origin.contents[0..=stop].iter().map(|c| c.fee()).sum();
        context.set_balance(&opg.source, &(initial_balance - total_fees))?;
    } else {
        context.set_counter(&opg.source, &opg.last_counter)?;
    }

    context.commit()?;
    context.log(format!("Operation applied: {:?}", opg.hash));

    Ok(OperationReceipt {
        protocol: Some(ProtocolHash::new(PROTOCOL.into())?),
        chain_id: Some(ChainId::new(CHAIN_ID.into())?),
        hash: Some(opg.hash.to_owned()),
        branch: opg.origin.branch.clone(),
        signature: Some(opg.origin.signature.clone()),
        contents
    })
}

#[cfg(test)]
mod test {
    use crate::context::{Context, ephemeral::EphemeralContext};
    use crate::Result;
    use crate::validator::ManagerOperation;
    use tezos_operation::{
        operations::{SignedOperation, Transaction}
    };
    use tezos_core::types::{
        encoded::{ImplicitAddress, Address},
        mutez::Mutez,
        number::Nat
    };
    use tezos_rpc::models::operation::{
        operation_result::OperationResultStatus
    };

    use super::{execute_operation, get_status};

    #[test]
    fn test_skipped_backtracked() -> Result<()> {
        let mut context = EphemeralContext::new();

        let source = ImplicitAddress::try_from("tz1V3dHSCJnWPRdzDmZGCZaTMuiTmbtPakmU").unwrap();
        let destination = Address::try_from("tz1NEgotHhj4fkm8AcwquQqQBrQsAMRUg86c").unwrap();

        context.set_balance(&source, &Mutez::from(4000u32))?;
        context.set_counter(&source, &Nat::try_from("1").unwrap())?;
        context.commit()?;

        macro_rules! make_tx {
            ($cnt: expr) => {
                Transaction {
                    source: source.clone(),
                    counter: $cnt.into(),
                    fee: 1000u32.into(),
                    gas_limit: 0u32.into(),
                    storage_limit: 0u32.into(),
                    amount: 1000u32.into(),
                    destination: destination.clone(),
                    parameters: None
                }
            };
        }

        let opg = ManagerOperation {
            hash: "ooKsoMe48CCt1ERrk5DgnSovFazhm53yfAYbwxNQmjWVtbNzLME".try_into().unwrap(),
            origin: SignedOperation::new(
                "BMNvSHmWUkdonkG2oFwwQKxHUdrYQhUXqxLaSRX9wjMGfLddURC".try_into().unwrap(),
                vec![make_tx!(2u32).into(), make_tx!(3u32).into(), make_tx!(4u32).into()], 
                "sigw1WNdYweqz1c7zKcvZFHQ18swSv4HBWje5quRmixxitPk7z8jtY63qXgKLPVfTM6XGxExPatBWJP44Bknyu3hDHDKJZgY".try_into().unwrap()
            ),
            last_counter: 4u32.into(),
            source: source.clone(),
            total_fees: 3000u32.into(),
            total_spent: 0u32.into()  // <-- not true, fot the sake of the test
        };

        let receipt = execute_operation(&mut context, &opg)?;
        // println!("{:#?}", receipt);
        assert_eq!(get_status(&receipt.contents[0]).expect("Backtracked"), OperationResultStatus::Backtracked);
        assert_eq!(get_status(&receipt.contents[1]).expect("Failed"), OperationResultStatus::Failed);
        assert_eq!(get_status(&receipt.contents[2]).expect("Skipped"), OperationResultStatus::Skipped);
        
        Ok(())
    }
}