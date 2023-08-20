// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use std::fmt::Display;

use tezos_core::types::encoded::{Address, ImplicitAddress};
use tezos_michelson::michelson::types::Type;

use crate::types::{BigMapDiff, InternalContent, OperationItem};
use crate::Result;

use super::{MutezItem, StackItem};

impl OperationItem {
    pub fn new(
        destination: Address,
        param: StackItem,
        param_type: Type,
        amount: MutezItem,
        source: ImplicitAddress,
    ) -> Self {
        Self {
            destination,
            param: Box::new(param),
            param_type,
            amount,
            source,
            big_map_diff: Vec::new(),
        }
    }

    pub fn into_content(self) -> Result<InternalContent> {
        let content = InternalContent::Transaction {
            destination: self.destination,
            parameter: self.param.into_micheline(&self.param_type)?,
            amount: self.amount.try_into()?,
            source: self.source.clone(),
        };

        Ok(content)
    }

    pub fn aggregate_diff(&mut self, big_map_diff: &mut Vec<BigMapDiff>) {
        big_map_diff.append(&mut self.big_map_diff)
    }
}

impl PartialEq for OperationItem {
    fn eq(&self, other: &Self) -> bool {
        // For testing purposes
        self.destination == other.destination
            && self.param == other.param
            && self.param_type == other.param_type
            && self.amount == other.amount
            && self.source == other.source
    }
}

impl Display for OperationItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Operation")
    }
}
