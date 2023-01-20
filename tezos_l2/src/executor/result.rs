use derive_more::{From, TryInto};
use tezos_core::types::encoded::{Address, Encoded};
use tezos_operation::operations::{Origination, Reveal, Transaction};
use tezos_rpc::models::operation::operation_result::{
    operations::origination::OriginationOperationResult, operations::reveal::RevealOperationResult,
    operations::transaction::InternalTransactionOperationResult,
    operations::transaction::TransactionOperationResult, operations::InternalOperationResult,
    OperationResultStatus,
};
use tezos_rpc::models::operation::{
    kind::OperationKind,
    operation_contents_and_result::{
        origination::{Origination as OriginationReceipt, OriginationMetadata},
        reveal::{Reveal as RevealReceipt, RevealMetadata},
        transaction::{Transaction as TransactionReceipt, TransactionMetadata},
    },
    OperationContent as OperationContentAndResult,
};

use crate::executor::balance_updates::BalanceUpdates;

#[derive(Debug, Clone, TryInto, From)]
pub enum ExecutionResult {
    Transaction {
        content: Transaction,
        sender: Option<Address>,
        result: TransactionOperationResult,
        internal_results: Vec<ExecutionResult>,
    },
    Origination {
        content: Origination,
        result: OriginationOperationResult,
    },
    Reveal {
        content: Reveal,
        result: RevealOperationResult,
    },
}

impl ExecutionResult {
    pub fn ok(&self) -> bool {
        let status = match self {
            Self::Transaction {
                content: _,
                sender: _,
                result,
                internal_results: _,
            } => result.status,
            Self::Origination { content: _, result } => result.status,
            Self::Reveal { content: _, result } => result.status,
        };

        match status {
            OperationResultStatus::Applied => true,
            OperationResultStatus::Skipped => true,
            _ => false,
        }
    }

    pub fn backtrack(&mut self) {
        match self {
            Self::Transaction {
                content: _,
                sender: _,
                result,
                internal_results: _,
            } => result.status = OperationResultStatus::Backtracked,
            Self::Origination { content: _, result } => {
                result.status = OperationResultStatus::Backtracked
            }
            Self::Reveal { content: _, result } => {
                result.status = OperationResultStatus::Backtracked
            }
        }
    }

    pub fn aggregate_internals(self, internals: &mut Vec<InternalOperationResult>) {
        match self {
            Self::Transaction {
                content,
                sender,
                result,
                internal_results,
            } => {
                internals.push(InternalOperationResult::Transaction(
                    InternalTransactionOperationResult {
                        kind: OperationKind::Transaction,
                        amount: content.amount,
                        destination: content.destination,
                        nonce: internals.len() as u16,
                        parameters: content.parameters.map(|p| p.into()),
                        source: sender.unwrap_or(content.source.into()),
                        result: Some(result),
                    },
                ));
                internal_results
                    .into_iter()
                    .for_each(|r| r.aggregate_internals(internals));
            }
            _ => unimplemented!(),
        }
    }
}

impl Into<OperationContentAndResult> for ExecutionResult {
    fn into(self) -> OperationContentAndResult {
        match self {
            Self::Reveal { content, result } => OperationContentAndResult::Reveal(RevealReceipt {
                metadata: Some(RevealMetadata {
                    operation_result: result,
                    balance_updates: BalanceUpdates::fee(content.source.value(), &content.fee),
                }),
                ..content.into()
            }),
            Self::Origination { content, result } => {
                OperationContentAndResult::Origination(OriginationReceipt {
                    metadata: Some(OriginationMetadata {
                        operation_result: result,
                        balance_updates: BalanceUpdates::fee(content.source.value(), &content.fee),
                    }),
                    ..content.into()
                })
            }
            Self::Transaction {
                content,
                sender: _,
                result,
                internal_results,
            } => {
                let mut internals = Vec::new();
                internal_results
                    .into_iter()
                    .for_each(|r| r.aggregate_internals(&mut internals));

                OperationContentAndResult::Transaction(TransactionReceipt {
                    metadata: Some(TransactionMetadata {
                        operation_result: result,
                        balance_updates: BalanceUpdates::fee(content.source.value(), &content.fee),
                        internal_operation_results: internals,
                    }),
                    ..content.into()
                })
            }
        }
    }
}
