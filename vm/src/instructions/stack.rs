use tezos_michelson::michelson::data::instructions::{Dig, Drop, Dug, Dup, Push, Swap};

use crate::{
    err_unsupported, interpreter::PureInterpreter, stack::Stack, types::StackItem, Result,
};

impl PureInterpreter for Push {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        // TODO: check if pushable
        let item = StackItem::from_data(*self.value.to_owned(), &self.r#type)?;
        stack.push(item)
    }
}

impl PureInterpreter for Drop {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let count: usize = match &self.n {
            Some(n) => n.to_integer()?,
            None => 1,
        };
        for _ in 0..count {
            stack.pop()?;
        }
        Ok(())
    }
}

impl PureInterpreter for Dup {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let n: usize = match &self.n {
            Some(n) => n.to_integer()?,
            None => 1,
        };
        if n == 0 {
            return err_unsupported!("DUP 0");
        }
        // TODO: check if copyable
        let res = stack.dup_at(n - 1)?;
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
        let item = stack.pop_at(self.n.to_integer()?)?;
        stack.push(item)
    }
}

impl PureInterpreter for Dug {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let item = stack.pop()?;
        stack.push_at(self.n.to_integer()?, item)
    }
}
