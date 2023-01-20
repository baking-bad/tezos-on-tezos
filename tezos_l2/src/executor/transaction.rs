use context::{ExecutorContext, InterpreterContext};
use tezos_core::types::encoded::{Address, Encoded};
use tezos_michelson::micheline::Micheline;
use tezos_operation::operations::{OperationContent, Transaction};
use tezos_rpc::models::operation::{
    operation_result::operations::transaction::TransactionOperationResult,
    operation_result::OperationResultStatus,
};

use crate::{
    executor::balance_updates::BalanceUpdates,
    executor::contract::{execute_contract, expand_content, ContractOutput},
    executor::lazy_diff::LazyDiff,
    executor::result::ExecutionResult,
    executor::rpc_errors::RpcErrors,
    Error, Result,
};

pub fn execute_transaction(
    context: &mut (impl ExecutorContext + InterpreterContext),
    transaction: &Transaction,
    sender: Option<Address>,
    skip: bool,
) -> Result<ExecutionResult> {
    let mut errors = RpcErrors::new();
    let mut balance_updates = BalanceUpdates::new();
    let mut lazy_diff = LazyDiff::new();
    let mut storage: Option<Micheline> = None;
    let mut internal_results: Vec<ExecutionResult> = Vec::new();

    macro_rules! result {
        ($status: ident) => {{
            let applied = OperationResultStatus::$status == OperationResultStatus::Applied;
            Ok(ExecutionResult::Transaction {
                content: transaction.clone(),
                result: TransactionOperationResult {
                    status: OperationResultStatus::$status,
                    consumed_milligas: if applied { Some("0".into()) } else { None },
                    lazy_storage_diff: lazy_diff.into(),
                    balance_updates: balance_updates.into(),
                    errors: errors.into(),
                    storage,
                    big_map_diff: None,
                    consumed_gas: None,
                    storage_size: None,
                    paid_storage_size_diff: None,
                    allocated_destination_contract: None,
                    originated_contracts: None, // TODO: copypaste?
                },
                sender,
                internal_results,
            })
        }};
    }

    if skip {
        return result!(Skipped);
    }

    let balance = match balance_updates.transfer(
        context,
        transaction.source.value(),
        transaction.destination.value(),
        &transaction.amount,
    ) {
        Ok((_, balance)) => balance,
        Err(Error::BalanceTooLow { balance }) => {
            errors.balance_too_low(&transaction.amount, &balance, transaction.source.value());
            return result!(Failed);
        }
        Err(err) => return Err(err),
    };

    if let Address::Implicit(_) = transaction.destination {
        return result!(Applied);
    }

    let internal_operations: Vec<OperationContent> =
        match execute_contract(context, transaction, sender.clone(), balance) {
            Ok(ContractOutput::Return(ret)) => {
                storage = Some(ret.storage);
                lazy_diff.update(ret.big_map_diff)?;
                ret.operations.into_iter().map(expand_content).collect()
            }
            Ok(ContractOutput::Error(err)) => {
                // TODO: rpc error
                return result!(Failed);
            }
            Err(err) => return Err(err),
        };

    for operation in internal_operations {
        match operation {
            OperationContent::Transaction(tx) => {
                match execute_transaction(
                    context,
                    &tx,
                    Some(transaction.destination.clone()),
                    false,
                ) {
                    Ok(res) => {
                        if !res.ok() {
                            internal_results.iter_mut().for_each(|r| r.backtrack());
                            internal_results.push(res);
                            return result!(Backtracked);
                        }
                        internal_results.push(res);
                    }
                    Err(err) => return Err(err),
                }
            }
            _ => return Err(Error::OperationKindUnsupported),
        }
    }

    result!(Applied)
}

#[cfg(test)]
mod test {
    use context::{EphemeralContext, ExecutorContext};
    use tezos_core::types::mutez::Mutez;
    use tezos_operation::operations::Transaction;

    use super::*;
    use crate::Result;

    #[test]
    fn test_transaction_applied() -> Result<()> {
        let mut context = EphemeralContext::new();

        let source = "tz1V3dHSCJnWPRdzDmZGCZaTMuiTmbtPakmU";
        let destination = "tz1NEgotHhj4fkm8AcwquQqQBrQsAMRUg86c";

        context.set_balance(source, &Mutez::from(1000000000u32))?;

        let transaction = Transaction {
            source: source.try_into()?,
            counter: 200000u32.into(),
            fee: 1000u32.into(),
            gas_limit: 0u32.into(),
            storage_limit: 0u32.into(),
            amount: 500000000u32.into(),
            destination: destination.try_into()?,
            parameters: None,
        };

        let res = execute_transaction(&mut context, &transaction, None, false);
        assert!(res.is_ok());
        assert!(res.unwrap().ok());

        assert_eq!(
            context.get_balance(&source)?.unwrap(),
            Mutez::from(1000000000u32 - 500000000u32)
        );
        assert_eq!(
            context.get_balance(&destination)?.unwrap(),
            Mutez::from(500000000u32)
        );

        Ok(())
    }
}
