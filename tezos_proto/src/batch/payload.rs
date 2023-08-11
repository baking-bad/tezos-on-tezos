// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use std::collections::BTreeMap;

use tezos_core::types::encoded::{OperationHash, OperationListListHash};
use tezos_operation::operations::SignedOperation;

use crate::Result;

use super::header::BatchHeader;

pub struct BatchPayload {
    pub header: BatchHeader,
    pub operations: BTreeMap<OperationHash, SignedOperation>,
}

impl BatchPayload {
    pub fn operation_list_list_hash(&self) -> Result<OperationListListHash> {
        let operation_hashes: Vec<OperationHash> = self.operations.keys().cloned().collect();
        Ok(OperationListListHash::try_from(vec![
            vec![],
            vec![],
            vec![],
            operation_hashes,
        ])?)
    }
}
