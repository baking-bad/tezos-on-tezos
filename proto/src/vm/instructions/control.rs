use tezos_michelson::michelson::data::instructions::{
    Dip, Sequence, FailWith, If, IfCons, IfLeft, IfNone, Loop, LoopLeft, Map, Iter
};

use crate::{
    Result,
    Error,
    error::InterpreterError,
    vm::interpreter::{Interpreter, TransactionScope, PureInterpreter},
    vm::types::{StackItem, ListItem, MapItem},
    vm::stack::Stack,
    context::Context,
    pop_cast,
    err_type,
    trace_enter,
    trace_exit
};

impl Interpreter for Sequence {
    fn execute(&self, stack: &mut Stack, tx_scope: &TransactionScope, global_ctx: &mut impl Context) -> Result<()> {
        for instr in self.instructions() {
            instr.execute(stack, tx_scope, global_ctx)?;
        }
        Ok(())
    }
}

impl Interpreter for Dip {
    fn execute(&self, stack: &mut Stack, tx_scope: &TransactionScope, global_ctx: &mut impl Context) -> Result<()> {
        let count: usize = match &self.n {
            Some(n) => n.to_integer()?,
            None => 1
        };
        stack.protect(count)?;
        self.instruction.execute(stack, tx_scope, global_ctx)?;
        stack.restore(count)
    }
}

impl PureInterpreter for FailWith {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let with = stack.pop()?;
        let ty = with.get_type()?;
        Err(Error::InterpreterError(InterpreterError::ScriptFailed { with: with.into_micheline(&ty)? }))
    }
}

impl Interpreter for If {
    fn execute(&self, stack: &mut Stack, tx_scope: &TransactionScope, global_ctx: &mut impl Context) -> Result<()> {
        let cond = pop_cast!(stack, Bool);
        let branch = if cond.is_true() {
            trace_exit!("Yes");
            &self.if_branch
        } else {
            trace_exit!("Else");
            &self.else_branch
        };        
        branch.execute(stack, tx_scope, global_ctx)
    }
}

impl Interpreter for IfCons {
    fn execute(&self, stack: &mut Stack, tx_scope: &TransactionScope, global_ctx: &mut impl Context) -> Result<()> {
        let list = pop_cast!(stack, List);
        let branch = if list.len() > 0 {
            let (head, tail) = list.split_head()?;
            stack.push(tail.into())?;
            stack.push(head)?;
            trace_exit!("Yes");
            &self.if_branch
        } else {
            trace_exit!("Else");
            &self.else_branch
        };
        branch.execute(stack, tx_scope, global_ctx)
    }
}

impl Interpreter for IfLeft {
    fn execute(&self, stack: &mut Stack, tx_scope: &TransactionScope, global_ctx: &mut impl Context) -> Result<()> {
        let or = pop_cast!(stack, Or);
        let cond = or.is_left();
        stack.push(or.unwrap())?;
        let branch = if cond {
            trace_exit!("Yes");
            &self.if_branch
        } else {
            trace_exit!("Else");
            &self.else_branch
        };
        branch.execute(stack, tx_scope, global_ctx)
    }
}

impl Interpreter for IfNone {
    fn execute(&self, stack: &mut Stack, tx_scope: &TransactionScope, global_ctx: &mut impl Context) -> Result<()> {
        let option = pop_cast!(stack, Option);
        let branch = match option.unwrap() {
            None => {
                trace_exit!("Yes");
                &self.if_branch
            },
            Some(item) => {
                stack.push(item)?;
                trace_exit!("Else");
                &self.else_branch
            }
        };
        branch.execute(stack, tx_scope, global_ctx)
    }
}

impl Interpreter for Loop {
    fn execute(&self, stack: &mut Stack, tx_scope: &TransactionScope, global_ctx: &mut impl Context) -> Result<()> {
        loop {
            let cond = pop_cast!(stack, Bool);
            if cond.is_true() {
                trace_enter!("Step");
                let res = self.body.execute(stack, tx_scope, global_ctx);
                trace_exit!(res.as_ref().err());
                res?
            } else {
                break Ok(())
            }
        }
    }
}

impl Interpreter for LoopLeft {
    fn execute(&self, stack: &mut Stack, tx_scope: &TransactionScope, global_ctx: &mut impl Context) -> Result<()> {
        loop {
            let or = pop_cast!(stack, Or);
            let cond = or.is_left();
            stack.push(or.unwrap())?;
            if cond {
                trace_enter!("Step");
                let res = self.body.execute(stack, tx_scope, global_ctx);
                trace_exit!(res.as_ref().err());
                res?
            } else {
                break Ok(())
            }
        }
    }
}

impl Interpreter for Map {
    fn execute(&self, stack: &mut Stack, tx_scope: &TransactionScope, global_ctx: &mut impl Context) -> Result<()> {
        let src = stack.pop()?;

        let mut process = |input: Vec<StackItem>| -> Result<Vec<StackItem>> {
            let mut output: Vec<StackItem> = Vec::with_capacity(input.len());
            for item in input {
                stack.push(item)?;
                trace_enter!("Step");
                let res = self.expression.execute(stack, tx_scope, global_ctx);
                trace_exit!(res.as_ref().err());
                res?;
                output.push(stack.pop()?);
            }
            Ok(output)
        };

        let res = match src {
            StackItem::List(list) => {
                let (input, val_type) = list.unwrap();
                let output = if input.is_empty() { vec![] } else { process(input)? };
                Ok(ListItem::new(output, val_type).into())
            },
            StackItem::Map(map) => {
                let keys = map.get_keys();
                let (input, (key_type, mut val_type)) = map.unwrap();
                let output: Vec<(StackItem, StackItem)> = if input.is_empty() {
                    vec![]
                } else {
                    let values = process(input)?;
                    val_type = values.first().unwrap().get_type()?;
                    keys.into_iter().zip(values.into_iter()).collect()
                };
                Ok(MapItem::new(output, key_type, val_type).into())
            },
            item => err_type!("ListItem | MapItem", item)
        };
        stack.push(res?)
    }
}

impl Interpreter for Iter {
    fn execute(&self, stack: &mut Stack, tx_scope: &TransactionScope, global_ctx: &mut impl Context) -> Result<()> {
        let input = match stack.pop()? {
            StackItem::Set(set) => set.unwrap().0,
            StackItem::List(list) => list.unwrap().0,
            StackItem::Map(map) => map.unwrap().0,
            item => return err_type!("SetItem | ListItem | MapItem", item)
        };
        for item in input {
            stack.push(item)?;
            trace_enter!("Step");
            let res = self.expression.execute(stack, tx_scope, global_ctx);
            trace_exit!(res.as_ref().err());
            res?;
        }
        Ok(())
    }
}