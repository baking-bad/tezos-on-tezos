// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use std::borrow::Borrow;

use tezos_michelson::michelson::data::instructions::{Dig, Drop, Dug, Dup, Push, Swap};

use crate::{
    err_unsupported,
    interpreter::{Interpreter, PureInterpreter, TicketStorage},
    stack::Stack,
    types::StackItem,
    InterpreterContext, OperationScope, Result,
};

impl PureInterpreter for Push {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        // TODO: check if pushable
        let item = StackItem::from_data(*self.value.to_owned(), &self.r#type)?;
        stack.push(item)
    }
}

impl Interpreter for Drop {
    fn execute(
        &self,
        stack: &mut Stack,
        scope: &OperationScope,
        context: &mut impl InterpreterContext,
    ) -> Result<()> {
        let count: usize = match &self.n {
            Some(n) => n.try_into()?,
            None => 1,
        };
        for _ in 0..count {
            let item = stack.pop()?;
            item.drop_tickets(&scope.self_address, context)?;
        }
        Ok(())
    }
}

impl PureInterpreter for Dup {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let n: usize = match &self.n {
            Some(n) => n.try_into()?,
            None => 1,
        };
        if n == 0 {
            return err_unsupported!("DUP 0");
        }
        // TODO: check if copyable
        let res = stack.dup_at(n - 1)?;

        if res.has_tickets() {
            return err_unsupported!("proto.alpha.michelson_v1.non_dupable_type");
        }

        stack.push(res)
    }
}

impl PureInterpreter for Swap {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let item = stack.pop()?;
        stack.push_at(1, item)?;
        stack.top()
    }
}

impl PureInterpreter for Dig {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let item = stack.pop_at(self.n.borrow().try_into()?)?;
        stack.push(item)
    }
}

impl PureInterpreter for Dug {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let item = stack.pop()?;
        stack.push_at(self.n.borrow().try_into()?, item)
    }
}
