// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use std::fmt::Display;

use tezos_michelson::michelson::types::{self, Type};

use crate::{interpreter::TicketStorage, types::TicketItem, Result};

use super::{
    AddressItem, BigMapItem, ListItem, MapItem, NatItem, OperationItem, OptionItem, OrItem,
    PairItem, StackItem,
};

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

impl TicketStorage for StackItem {
    fn has_tickets(&self) -> bool {
        match self {
            StackItem::BigMap(item) => item.has_tickets(),
            StackItem::Option(item) => item.has_tickets(),
            StackItem::Or(item) => item.has_tickets(),
            StackItem::Pair(item) => item.has_tickets(),
            StackItem::List(item) => item.has_tickets(),
            StackItem::Map(item) => item.has_tickets(),
            StackItem::Ticket(item) => item.has_tickets(),
            StackItem::Operation(item) => item.has_tickets(),
            _ => false,
        }
    }

    fn iter_tickets(&self, action: &mut impl FnMut(&TicketItem) -> Result<()>) -> Result<()> {
        match self {
            StackItem::BigMap(item) => item.iter_tickets(action),
            StackItem::Option(item) => item.iter_tickets(action),
            StackItem::Or(item) => item.iter_tickets(action),
            StackItem::Pair(item) => item.iter_tickets(action),
            StackItem::List(item) => item.iter_tickets(action),
            StackItem::Map(item) => item.iter_tickets(action),
            StackItem::Ticket(item) => item.iter_tickets(action),
            StackItem::Operation(item) => item.iter_tickets(action),
            _ => Ok(()),
        }
    }
}

impl TicketStorage for TicketItem {
    fn has_tickets(&self) -> bool {
        true
    }

    fn iter_tickets(&self, action: &mut impl FnMut(&TicketItem) -> Result<()>) -> Result<()> {
        action(self)
    }
}

impl TicketStorage for BigMapItem {
    fn has_tickets(&self) -> bool {
        match self {
            BigMapItem::Diff(val) => todo!(),
            BigMapItem::Map(val) => val.has_tickets(),
            BigMapItem::Ptr(_) => false,
        }
    }

    fn iter_tickets(&self, action: &mut impl FnMut(&TicketItem) -> Result<()>) -> Result<()> {
        match self {
            BigMapItem::Diff(val) => todo!(),
            BigMapItem::Map(val) => val.iter_tickets(action),
            BigMapItem::Ptr(_) => Ok(()),
        }
    }
}

impl TicketStorage for OptionItem {
    fn has_tickets(&self) -> bool {
        match self {
            Self::None(_) => false,
            Self::Some(val) => val.has_tickets(),
        }
    }

    fn iter_tickets(&self, action: &mut impl FnMut(&TicketItem) -> Result<()>) -> Result<()> {
        match self {
            Self::None(_) => Ok(()),
            Self::Some(val) => val.iter_tickets(action),
        }
    }
}

impl TicketStorage for OrItem {
    fn has_tickets(&self) -> bool {
        match self {
            Self::Left(var) => var.value.has_tickets(),
            Self::Right(var) => var.value.has_tickets(),
        }
    }

    fn iter_tickets(&self, action: &mut impl FnMut(&TicketItem) -> Result<()>) -> Result<()> {
        match self {
            Self::Left(var) => var.value.iter_tickets(action),
            Self::Right(var) => var.value.iter_tickets(action),
        }
    }
}

impl TicketStorage for PairItem {
    fn has_tickets(&self) -> bool {
        self.0 .0.has_tickets() || self.0 .1.has_tickets()
    }

    fn iter_tickets(&self, action: &mut impl FnMut(&TicketItem) -> Result<()>) -> Result<()> {
        self.0 .0.iter_tickets(action)?;
        self.0 .1.iter_tickets(action)
    }
}

impl TicketStorage for ListItem {
    fn has_tickets(&self) -> bool {
        for e in &self.outer_value {
            if e.has_tickets() {
                return true;
            }
        }
        false
    }

    fn iter_tickets(&self, action: &mut impl FnMut(&TicketItem) -> Result<()>) -> Result<()> {
        self.outer_value
            .iter()
            .map(|e| e.iter_tickets(action))
            .collect()
    }
}

impl TicketStorage for MapItem {
    fn has_tickets(&self) -> bool {
        for (k, v) in &self.outer_value {
            if k.has_tickets() || v.has_tickets() {
                return true;
            }
        }
        false
    }

    fn iter_tickets(&self, action: &mut impl FnMut(&TicketItem) -> Result<()>) -> Result<()> {
        self.outer_value
            .iter()
            .map(|(k, v)| -> Result<()> {
                k.iter_tickets(action)?;
                v.iter_tickets(action)
            })
            .collect()
    }
}

impl TicketStorage for OperationItem {
    fn has_tickets(&self) -> bool {
        self.param.has_tickets()
    }

    fn iter_tickets(&self, action: &mut impl FnMut(&TicketItem) -> Result<()>) -> Result<()> {
        self.param.iter_tickets(action)
    }
}
