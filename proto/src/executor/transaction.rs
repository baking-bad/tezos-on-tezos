use tezos_operation::operations::Transaction;
use tezos_rpc::models::operation::{
    operation_contents_and_result::transaction::{
        Transaction as TransactionReceipt, TransactionMetadata,
    },
    operation_result::operations::transaction::TransactionOperationResult,
    operation_result::OperationResultStatus,
};

use crate::{
    context::Context,
    error::{Result, RpcErrors},
    executor::balance_update::BalanceUpdates,
};

const DEFAULT_RESULT: TransactionOperationResult = TransactionOperationResult {
    status: OperationResultStatus::Skipped,
    storage: None,
    big_map_diff: None,
    balance_updates: None,
    originated_contracts: None,
    consumed_gas: None,
    consumed_milligas: None,
    storage_size: None,
    paid_storage_size_diff: None,
    allocated_destination_contract: None,
    lazy_storage_diff: None,
    errors: None,
};

pub fn skip_transaction(transaction: Transaction) -> TransactionReceipt {
    TransactionReceipt {
        metadata: Some(TransactionMetadata {
            operation_result: DEFAULT_RESULT,
            internal_operation_results: vec![],
            balance_updates: vec![],
        }),
        ..transaction.into()
    }
}

pub fn execute_transaction(
    context: &mut impl Context,
    transaction: &Transaction,
) -> Result<TransactionReceipt> {
    if transaction.parameters.is_some() {
        todo!("Support smart contract calls");
    }

    let mut dst_balance = context
        .get_balance(&transaction.destination)?
        .unwrap_or(0u32.into());

    let mut src_balance = context
        .get_balance(&transaction.source)?
        .expect("Source balance has to be checked by validator");

    let mut errors = RpcErrors::new();
    let mut balance_updates = BalanceUpdates::new();
    let charges = BalanceUpdates::fee(&transaction.source, &transaction.fee);

    macro_rules! make_receipt {
        ($a: expr) => {
            TransactionReceipt {
                metadata: Some(TransactionMetadata {
                    operation_result: TransactionOperationResult {
                        status: $a,
                        balance_updates: balance_updates.into(),
                        consumed_milligas: Some("0".into()),
                        allocated_destination_contract: Some(false),
                        errors: errors.into(),
                        ..DEFAULT_RESULT
                    },
                    internal_operation_results: vec![],
                    balance_updates: charges.unwrap(),
                }),
                ..transaction.clone().into()
            }
        };
    }

    if src_balance < transaction.amount {
        errors.balance_too_low(&transaction.amount, &src_balance, &transaction.source);
        return Ok(make_receipt!(OperationResultStatus::Failed));
    } else {
        src_balance -= transaction.amount;
        dst_balance += transaction.amount;
        balance_updates.spend(&transaction.source, &transaction.amount);
        balance_updates.topup(&transaction.destination, &transaction.amount);
    }

    context.set_balance(&transaction.source, &src_balance)?;
    context.set_balance(&transaction.destination, &dst_balance)?;
    Ok(make_receipt!(OperationResultStatus::Applied))
}

#[cfg(test)]
mod test {
    use crate::context::{ephemeral::EphemeralContext, Context};
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
