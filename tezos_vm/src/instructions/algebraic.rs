use tezos_michelson::michelson::{
    data::instructions::{Car, Cdr, Left, None, Pair, Right, Some, Unit, Unpair},
    data::Nat,
};

use crate::{
    err_mismatch,
    interpreter::PureInterpreter,
    pop_cast,
    stack::Stack,
    types::{OptionItem, OrItem, PairItem, StackItem},
    Result,
};

impl PureInterpreter for Unit {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        stack.push(StackItem::Unit(().into()))
    }
}

impl PureInterpreter for Car {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let pair = pop_cast!(stack, Pair);
        let (first, _) = pair.unpair();
        stack.push(first)
    }
}

impl PureInterpreter for Cdr {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let pair = pop_cast!(stack, Pair);
        let (_, second) = pair.unpair();
        stack.push(second)
    }
}

fn parse_arity(n: &Option<Nat>) -> Result<usize> {
    let n: usize = match n {
        Some(n) => n.try_into()?,
        None => 2,
    };
    if n < 2 {
        return err_mismatch!(">=2 args", n);
    }
    Ok(n)
}

impl PureInterpreter for Pair {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let n = parse_arity(&self.n)?;
        let mut items: Vec<StackItem> = Vec::with_capacity(n);
        for _ in 0..n {
            items.push(stack.pop()?);
        }

        let pair = PairItem::from_items(items)?;
        stack.push(pair.into())
    }
}

impl PureInterpreter for Unpair {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let pair = pop_cast!(stack, Pair);
        let n = parse_arity(&self.n)?;
        let items = pair.into_items(n)?;
        for item in items.into_iter().rev() {
            stack.push(item)?;
        }
        Ok(())
    }
}

impl PureInterpreter for None {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let item = OptionItem::none(&self.r#type);
        stack.push(item.into())
    }
}

impl PureInterpreter for Some {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let val = stack.pop()?;
        let item = OptionItem::some(val);
        stack.push(item.into())
    }
}

impl PureInterpreter for Left {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let left_val = stack.pop()?;
        let res = OrItem::left(left_val, self.r#type.clone());
        stack.push(res.into())
    }
}

impl PureInterpreter for Right {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let right_val = stack.pop()?;
        let res = OrItem::right(right_val, self.r#type.clone());
        stack.push(res.into())
    }
}
