// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use tezos_michelson::michelson::data::instructions::{
    Ticket, ReadTicket, SplitTicket, JoinTickets,
};

use crate::{
    err_mismatch,
    interpreter::{
        ContextInterpreter, InterpreterContext, PureInterpreter,
    },
    //pop_cast,
    stack::Stack,
    Result, types::{AddressItem, NatItem, StackItem, TicketItem, PairItem, OptionItem},
};

impl ContextInterpreter for Ticket {
    fn execute(&self, stack: &mut Stack, context: &mut impl InterpreterContext) -> Result<()> {
        let identifier = stack.pop()?;
        let amount = stack.pop()?;
        
        // TODO: compare amount with zero
        // TODO: convert StackItem identifier to Micheline
        // TODO: get Type for identifier
        // TODO: get self address
        // TODO: save balance info to context? 

        //stack.push(StackItem::Ticket(TicketItem::new()));
        Ok(())
    }
}

impl PureInterpreter for ReadTicket {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let ticket_item = stack.pop()?;
        let ticket = match ticket_item {
            StackItem::Ticket(ticket) => ticket,
            item => return err_mismatch!("Ticket", item)
        };

        let source = StackItem::Address(AddressItem::new(ticket.source));
        let identifier = ticket.identifier; // TODO: identifier to StackItem
        let amount = StackItem::Nat(NatItem::new(ticket.amount));
        
        let pair = PairItem::from_items(vec![source, identifier, amount])?;

        stack.push(StackItem::Pair(pair));
        stack.push(ticket_item); // return ticket back to stack
        Ok(())
    }
}

impl ContextInterpreter for SplitTicket {
    fn execute(&self, stack: &mut Stack, context: &mut impl InterpreterContext) -> Result<()> {
        let ticket = stack.pop()?; // ticket
        let split_pair = stack.pop()?; // pair nat nat

        // TODO: if n + m != ticket.amount or n == 0 or m == 0 return none
        stack.push(StackItem::Option(OptionItem::None()));

        // TODO: else return pair (ticket_n, ticket_m)
        stack.push(StackItem::Option(OptionItem::Some()));
        
        // TODO: update balance in context?

        Ok(())
    }
}

impl ContextInterpreter for JoinTickets {
    fn execute(&self, stack: &mut Stack, context: &mut impl InterpreterContext) -> Result<()> {
        let tickets = stack.pop()?; // tickets pair
        // TODO: get ticket_a
        // TODO: get ticket_b
        // TODO: compare sources and identifiers (and identifiers types?)

        // TODO: if ticket_a.source != ticket_b.source or ticket_a.identifier != ticket_b.identifier
        stack.push(StackItem::Option(OptionItem::None()));

        // TODO: OR otherwise return Some(ticket)
        stack.push(StackItem::Option(OptionItem::Some()));
        
        // TODO: update balance in context?

        Ok(())
    }
}