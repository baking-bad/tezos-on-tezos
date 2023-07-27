// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use std::fmt::Display;

use tezos_michelson::michelson::types::{self, Type};

use crate::{types::TicketItem, Result};

use super::{AddressItem, NatItem, StackItem};

impl TicketItem {
    pub fn new(source: AddressItem, identifier: StackItem, amount: NatItem) -> Self {
        Self {
            source: source,
            identifier: Box::new(identifier),
            amount: amount,
        }
    }

    pub fn get_type(&self) -> Result<Type> {
        Ok(types::ticket(self.identifier.get_type()?))
    }
}

impl Display for TicketItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "({:?} {:?} {})",
            self.source, self.identifier, self.amount
        ))
    }
}
