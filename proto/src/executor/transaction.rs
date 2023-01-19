use tezos_core::types::encoded::{Encoded, Address};
use tezos_michelson::micheline::Micheline;
use tezos_operation::operations::{Transaction, OperationContent};
use tezos_rpc::models::operation::{
    operation_result::operations::transaction::TransactionOperationResult,
    operation_result::OperationResultStatus,
};
use vm::interpreter::InterpreterContext;

use crate::{
    Result,
    Error,
    context::proto::ProtoContext,
    error::{RpcErrors},
    executor::balance_updates::BalanceUpdates,
    executor::result::ExecutionResult,
    executor::lazy_diff::LazyDiff,
    executor::contract::{execute_contract, expand_content}
};

pub fn execute_transaction(
    context: &mut (impl ProtoContext + InterpreterContext),
    transaction: &Transaction,
    skip: bool
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
                internal_results
            })
        }};
    }

    if skip {
        return result!(Skipped)
    }

    let balance = match balance_updates.transfer(
        context,
        transaction.source.value(),
        transaction.destination.value(),
        &transaction.amount
    ) {
        Ok((_, balance)) => balance,
        Err(Error::BalanceTooLow { balance }) => {
            errors.balance_too_low(&transaction.amount, &balance, transaction.source.value());
            return result!(Failed)
        },
        Err(err) => return Err(err)
    };

    if let Address::Implicit(_) = transaction.destination {
        return result!(Applied);
    }

    let internal_operations: Vec<OperationContent> = match execute_contract(context, transaction, balance) {
        Ok(ret) => {
            storage = Some(ret.storage);
            lazy_diff.update(ret.big_map_diff);
            ret.operations.into_iter().map(expand_content).collect()
        },
        Err(err) => {
            // TODO: runtime error
            return result!(Failed)
        }
    };

    for operation in internal_operations {
        match operation {
            OperationContent::Transaction(tx) => {
                match execute_transaction(context, &tx, false) {
                    Ok(res) => {
                        if !res.ok() {
                            internal_results.iter_mut().for_each(|r| r.backtrack(context));
                            internal_results.push(res);
                            return result!(Backtracked);
                        }
                        internal_results.push(res);
                    },
                    Err(err) => return Err(err)
                }
            },
            _ => return Err(Error::OperationKindUnsupported)
        }
    }
    
    result!(Applied)
}

#[cfg(test)]
mod test {
    use crate::context::ephemeral::EphemeralContext;
    use crate::Result;
    use tezos_core::types::{
        encoded::{Address, ImplicitAddress},
        mutez::Mutez,
    };
    use tezos_operation::operations::Transaction;

    use super::execute_transaction;

    #[test]
    fn test_transaction_applied() -> Result<()> {
        let mut context = EphemeralContext::new();

        let source = ImplicitAddress::try_from("tz1V3dHSCJnWPRdzDmZGCZaTMuiTmbtPakmU").unwrap();
        let destination =
            ImplicitAddress::try_from("tz1NEgotHhj4fkm8AcwquQqQBrQsAMRUg86c").unwrap();

        context.set_balance(&source, &Mutez::from(1000000000u32))?;

        let transaction = Transaction {
            source: source.clone(),
            counter: 200000u32.into(),
            fee: 1000u32.into(),
            gas_limit: 0u32.into(),
            storage_limit: 0u32.into(),
            amount: 500000000u32.into(),
            destination: Address::Implicit(destination.clone()),
            parameters: None,
        };

        let receipt = execute_transaction(&mut context, &transaction);
        assert!(receipt.is_ok());
        assert!(receipt.unwrap().metadata.is_some());

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
