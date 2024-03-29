// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use michelson_vm::interpreter::InterpreterContext;
use tezos_core::types::encoded::Encoded;
use tezos_operation::operations::OperationContent;
use tezos_rpc::models::operation::Operation as OperationReceipt;

use crate::{
    config::PROTOCOL,
    context::TezosContext,
    error::{Error, Result},
    executor::{
        balance_updates::BalanceUpdates, origination::execute_origination, reveal::execute_reveal,
        transaction::execute_transaction,
    },
    validator::operation::ValidOperation,
};

pub fn execute_operation(
    context: &mut (impl TezosContext + InterpreterContext),
    opg: &ValidOperation,
) -> Result<OperationReceipt> {
    context.check_no_pending_changes()?;

    let mut failed_idx: Option<usize> = None;
    let mut origination_index: i32 = 0;
    let mut results = Vec::new();

    BalanceUpdates::reserve(context, opg.source.value(), &opg.total_fees)?;

    for (i, content) in opg.origin.contents.iter().enumerate() {
        let skip = failed_idx.is_some();
        let result = match content {
            OperationContent::Reveal(reveal) => execute_reveal(context, reveal, skip)?,
            OperationContent::Origination(origination) => execute_origination(
                context,
                origination,
                &opg.hash,
                &mut origination_index,
                skip,
            )?,
            OperationContent::Transaction(transaction) => {
                execute_transaction(context, transaction, None, skip)?
            }
            _ => return Err(Error::OperationKindUnsupported),
        };

        if !skip && !result.ok() {
            failed_idx = Some(i);
            context.rollback();
            context.log(result.errors().join("\n"))
        }

        results.push(result);
    }

    if let Some(stop) = failed_idx {
        results[0..stop].iter_mut().for_each(|r| r.backtrack());

        let total_fees = opg.origin.contents[0..=stop].iter().map(|c| c.fee()).sum();
        BalanceUpdates::reserve(context, opg.source.value(), &total_fees)?;
    } else {
        // all applied, no rollbacks
        context.set_counter(opg.source.value(), opg.last_counter.clone())?;
    }

    context.commit()?;
    context.log(format!("Operation included: {}", opg.hash.value()));

    let head = context.get_head()?;
    Ok(OperationReceipt {
        protocol: Some(PROTOCOL.try_into()?),
        chain_id: Some(head.chain_id),
        hash: Some(opg.hash.to_owned()),
        branch: opg.origin.branch.clone(),
        signature: Some(opg.origin.signature.clone()),
        contents: results.into_iter().map(|r| r.into()).collect(),
    })
}

#[cfg(test)]
mod test {
    use tezos_core::types::{mutez::Mutez, number::Nat};
    use tezos_operation::operations::{SignedOperation, Transaction};
    use tezos_rpc::models::operation::{operation_result::OperationResultStatus, OperationContent};

    use super::*;
    use crate::{context::TezosEphemeralContext, validator::operation::ValidOperation, Result};

    macro_rules! get_status {
        ($receipt: expr) => {
            if let Some(metadata) = $receipt.metadata.as_ref() {
                return Ok(metadata.operation_result.status.clone());
            }
        };
    }

    fn get_status(receipt: &OperationContent) -> Result<OperationResultStatus> {
        match receipt {
            OperationContent::Reveal(reveal) => get_status!(reveal),
            OperationContent::Transaction(transaction) => get_status!(transaction),
            OperationContent::Origination(origination) => get_status!(origination),
            _ => return Err(Error::OperationKindUnsupported),
        }
        panic!("Operation metadata is missing: {:?}", receipt) // TODO: avoid panic?
    }

    #[test]
    fn test_skipped_backtracked() -> Result<()> {
        let mut context = TezosEphemeralContext::default();

        let source = "tz1V3dHSCJnWPRdzDmZGCZaTMuiTmbtPakmU";
        let destination = "tz1NEgotHhj4fkm8AcwquQqQBrQsAMRUg86c";

        context.set_balance(source, Mutez::from(4000u32))?;
        context.set_counter(source, Nat::try_from("1").unwrap())?;
        context.commit()?;

        macro_rules! make_tx {
            ($cnt: expr) => {
                Transaction {
                    source: source.try_into()?,
                    counter: $cnt.into(),
                    fee: 1000u32.into(),
                    gas_limit: 0u32.into(),
                    storage_limit: 0u32.into(),
                    amount: 1000u32.into(),
                    destination: destination.try_into()?,
                    parameters: None,
                }
            };
        }

        let opg = ValidOperation {
            hash: "ooKsoMe48CCt1ERrk5DgnSovFazhm53yfAYbwxNQmjWVtbNzLME"
                .try_into()
                .unwrap(),
            origin: SignedOperation::new(
                "BMNvSHmWUkdonkG2oFwwQKxHUdrYQhUXqxLaSRX9wjMGfLddURC"
                    .try_into()
                    .unwrap(),
                vec![make_tx!(2u32).into(), make_tx!(3u32).into(), make_tx!(4u32).into()],
                "sigw1WNdYweqz1c7zKcvZFHQ18swSv4HBWje5quRmixxitPk7z8jtY63qXgKLPVfTM6XGxExPatBWJP44Bknyu3hDHDKJZgY"
                    .try_into()
                    .unwrap(),
            ),
            last_counter: 4u32.into(),
            source: source.try_into()?,
            total_fees: 3000u32.into(),
            total_spent: 0u32.into(), // <-- not true, fot the sake of the test
        };

        let receipt = execute_operation(&mut context, &opg)?;
        //println!("{:#?}", receipt);
        assert_eq!(
            get_status(&receipt.contents[0]).expect("Backtracked"),
            OperationResultStatus::Backtracked
        );
        assert_eq!(
            get_status(&receipt.contents[1]).expect("Failed"),
            OperationResultStatus::Failed
        );
        assert_eq!(
            get_status(&receipt.contents[2]).expect("Skipped"),
            OperationResultStatus::Skipped
        );

        Ok(())
    }
}
