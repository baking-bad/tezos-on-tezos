// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use tezos_michelson::michelson::{
    data::instructions::{JoinTickets, ReadTicket, SplitTicket, Ticket},
    types,
};

use crate::{
    err_mismatch,
    interpreter::{PureInterpreter, ScopedInterpreter},
    pop_cast,
    stack::Stack,
    typechecker::check_type_comparable,
    types::{AddressItem, OptionItem, PairItem, StackItem, TicketItem},
    OperationScope, Result,
};

impl ScopedInterpreter for Ticket {
    fn execute(&self, stack: &mut Stack, scope: &OperationScope) -> Result<()> {
        let identifier = stack.pop()?;
        let identifier_ty = identifier.get_type()?;
        check_type_comparable(&identifier_ty)?;

        let amount = pop_cast!(stack, Nat);

        if amount.is_zero() {
            let ty = types::ticket(identifier_ty);
            return stack.push(StackItem::Option(OptionItem::None(ty)));
        }

        let ticket = TicketItem {
            source: AddressItem::new(scope.self_address.clone().into()),
            identifier: Box::new(identifier),
            amount: amount,
        };

        stack.push(StackItem::Option(OptionItem::Some(Box::new(ticket.into()))))
    }
}

impl PureInterpreter for ReadTicket {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let ticket = pop_cast!(stack, Ticket);

        let pair = PairItem::from_items(vec![
            ticket.source.clone().into(),
            *ticket.identifier.clone(),
            ticket.amount.clone().into(),
        ])?;

        stack.push(ticket.into())?; // return ticket back to stack
        stack.push(pair.into())
    }
}

impl PureInterpreter for SplitTicket {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let ticket = pop_cast!(stack, Ticket); // ticket
        let pair_n1_n2 = pop_cast!(stack, Pair); // pair nat nat

        let (n1, n2) = match pair_n1_n2.unpair() {
            (StackItem::Nat(n1), StackItem::Nat(n2)) => (n1, n2),
            (s1, s2) => {
                return err_mismatch!(
                    "Pair Nat Nat",
                    StackItem::Pair(PairItem::new(s1, s2))
                )
            }
        };

        if n1.is_zero() || n2.is_zero() || n1.clone() + n2.clone() != ticket.amount {
            let ty = types::pair(vec![types::nat(), types::nat()]);
            return stack.push(StackItem::Option(OptionItem::None(ty)));
        }

        let ticket_1 = TicketItem {
            source: ticket.source.clone(),
            identifier: ticket.identifier.clone(),
            amount: n1,
        };
        let ticket_2 = TicketItem {
            source: ticket.source,
            identifier: ticket.identifier,
            amount: n2,
        };
        let pair = PairItem::new(ticket_1.into(), ticket_2.into());

        stack.push(StackItem::Option(OptionItem::Some(Box::new(pair.into()))))
    }
}

impl PureInterpreter for JoinTickets {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let tickets = pop_cast!(stack, Pair); // tickets pair
        let (ticket_1, ticket_2) = match tickets.unpair() {
            (StackItem::Ticket(ticket_1), StackItem::Ticket(ticket_2)) => (ticket_1, ticket_2),
            (s1, s2) => {
                return err_mismatch!(
                    "Pair Ticket Ticket",
                    StackItem::Pair(PairItem::new(s1, s2))
                )
            }
        };

        if ticket_1.source != ticket_2.source || *ticket_1.identifier != *ticket_2.identifier {
            let ty = types::ticket(ticket_1.identifier.get_type()?);
            return stack.push(StackItem::Option(OptionItem::None(ty)));
        }

        let ticket = TicketItem {
            source: ticket_1.source,
            identifier: ticket_1.identifier,
            amount: ticket_1.amount + ticket_2.amount,
        };

        stack.push(StackItem::Option(OptionItem::Some(Box::new(ticket.into()))))
    }
}
