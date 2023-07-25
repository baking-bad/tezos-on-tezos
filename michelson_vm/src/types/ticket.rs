// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use std::fmt::Display;

use ibig::UBig;
use tezos_core::types::encoded::Address;
use tezos_michelson::{michelson::types::{self, Type}, micheline::Micheline};

use crate::{
    types::TicketItem,
    Result,
};

impl TicketItem {
    pub fn new(source: Address, identifier: Micheline, identifier_type: Type, amount: UBig) -> Self{
        Self {
            source,
            identifier,
            identifier_type,
            amount,
        }
    }

    pub fn get_type(&self) -> Result<Type> {
        Ok(types::ticket(self.identifier_type.clone()))
    }

}

impl Display for TicketItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({:?} {:?} {})", self.source, self.identifier, self.amount))
    }
}