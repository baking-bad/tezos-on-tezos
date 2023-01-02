use std::collections::VecDeque;

use crate::{
    error::{Result, InterpreterError},
    vm::types::StackItem
};

mod trace {
    use crate::vm::types::StackItem;
    use tezos_michelson::michelson::data::Instruction;

    static mut CALL_DEPTH: usize = 0;
    const DELIM: &str = "└";

    pub fn begin(instr: &Instruction) {        
        unsafe {
            eprintln!("{:-^width$} {:?}", DELIM, instr, width = CALL_DEPTH);
            CALL_DEPTH += 1;
        }
    }

    pub fn act(action: &str, item: &StackItem) {
        unsafe {
            eprintln!("{:-^width$} {} {:?}", DELIM, action, item, width = CALL_DEPTH);
            CALL_DEPTH += 1;
        }
    }

    pub fn end() {
        unsafe { CALL_DEPTH -= 1 }
    }
}

#[macro_export]
macro_rules! pop_cast {
    ($stack: expr, $var: ident) => {
        match $stack.pop()? {
            StackItem::$var(item) => item,
            item => return err_type!(stringify!($var), item)
        }
    };
}

pub struct Stack {
    items: VecDeque<StackItem>,
    protected: usize
}

impl Stack {
    pub fn new() -> Self {
        Self { items: VecDeque::new(), protected: 0 }
    }

    pub fn len(&self) -> usize {
        self.items.len() - self.protected
    }

    pub fn protect(&mut self, count: usize) -> Result<()> {
        if self.items.len() < count + self.protected {
            return Err(InterpreterError::BadStack { location: count }.into())
        }
        self.protected += count;
        Ok(())
    }

    pub fn restore(&mut self, count: usize) -> Result<()> {
        if self.protected < count {
            return Err(InterpreterError::BadStack { location: count }.into())
        }
        self.protected -= count;
        Ok(())
    }

    pub fn push_at(&mut self, depth: usize, item: StackItem) -> Result<()> {
        let depth = depth + self.protected;
        if self.items.len() < depth {
            return Err(InterpreterError::BadStack { location: depth }.into())
        }
        self.items.insert(depth, item);
        Ok(())
    }

    pub fn push(&mut self, item: StackItem) -> Result<()> {
        if self.protected > 0 {
            self.push_at(0, item)
        } else {
            self.items.push_front(item);
            Ok(())
        }
    }

    pub fn pop_at(&mut self, depth: usize) -> Result<StackItem> {
        match self.items.remove(depth + self.protected) {
            Some(item) => Ok(item),
            None => Err(InterpreterError::BadStack { location: depth }.into())
        }
    }

    pub fn pop(&mut self) -> Result<StackItem> {
        if self.protected > 0 {
            self.pop_at(0)
        } else {
            match self.items.pop_front() {
                Some(item) => Ok(item),
                None => Err(InterpreterError::BadStack { location: 0 }.into())
            }
        }
    }

    pub fn dup_at(&self, depth: usize) -> Result<StackItem> {
        match self.items.get(depth + self.protected) {
            Some(item) => Ok(item.clone()),
            None => Err(InterpreterError::BadStack { location: depth }.into())
        }
    }
}
