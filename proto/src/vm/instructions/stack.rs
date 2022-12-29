use tezos_michelson::michelson::data::instructions::{Push, Drop, Dup, Dig, Dug, Swap};

use crate::{
    Result,
    vm::interpreter::PureInterpreter,
    vm::types::StackItem,
    vm::stack::Stack
};

impl PureInterpreter for Push {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        stack.push(StackItem::from_data(*self.value.to_owned(), &self.r#type)?)
    }
}

impl PureInterpreter for Drop {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        match &self.n {
            Some(n) => stack.pop_at(n.to_integer()?)?,
            None => stack.pop()?
        };
        Ok(())
    }
}

impl PureInterpreter for Dup {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let depth: usize = match &self.n {
            Some(n) => n.to_integer()?,
            None => 0
        };
        stack.dup_at(depth)?;
        Ok(())
    }
}

impl PureInterpreter for Swap {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let item = stack.pop()?;
        stack.push_at(1, item)
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
