use tezos_operation::operations::{Transaction, Origination, Reveal};
use tezos_rpc::models::operation::{
    operation_result::{
        OperationResultStatus,
        operations::transaction::TransactionOperationResult,
        operations::origination::OriginationOperationResult,
        operations::reveal::RevealOperationResult
    },
};

use crate::{
    context::proto::ProtoContext
};

pub struct InternalTransaction {

}

pub enum ExecutionResult {
    Transaction {
        content: Transaction,
        result: TransactionOperationResult,
        internal_results: Vec<ExecutionResult>
    },
    Origination {
        content: Origination,
        result: OriginationOperationResult,
    },
    Reveal {
        content: Reveal,
        result: RevealOperationResult
    },
}

impl ExecutionResult {
    pub fn ok(&self) -> bool {
        match self {
            Self::Transaction { content: _, result, internal_results: _ } => {
                result.status == OperationResultStatus::Applied
            },
            Self::Origination { content: _, result } => {
                result.status == OperationResultStatus::Applied
            },
            Self::Reveal { content: _, result } => {
                result.status == OperationResultStatus::Applied
            }
        }
    }

    pub fn backtrack(&mut self, context: &mut impl ProtoContext) {
        match self {
            Self::Transaction { content: _, result, internal_results: _ } => {
                result.status = OperationResultStatus::Backtracked
            },
            Self::Origination { content: _, result } => {
                result.status = OperationResultStatus::Backtracked
            },
            Self::Reveal { content: _, result } => {
                result.status = OperationResultStatus::Backtracked
            },
        }
    }
}