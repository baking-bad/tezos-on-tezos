// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use std::collections::VecDeque;

use crate::{trace_log, trace_stack, types::StackItem, Error, Result};

#[macro_export]
macro_rules! pop_cast {
    ($stack: expr, $var: ident) => {
        match $stack.pop()? {
            StackItem::$var(item) => item,
            item => return err_mismatch!(stringify!($var), item),
        }
    };
}

pub struct Stack {
    items: VecDeque<StackItem>,
    protected: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
            protected: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn top(&self) -> Result<()> {
        if self.items.len() > self.protected {
            trace_stack!("Top", &self.items[self.protected], None);
        } else {
            trace_log!("Top", "empty stack");
        }
        Ok(())
    }

    pub fn protect(&mut self, count: usize) -> Result<()> {
        if self.items.len() < count + self.protected {
            return Err(Error::BadStack { location: count }.into());
        }
        self.protected += count;
        Ok(())
    }

    pub fn restore(&mut self, count: usize) -> Result<()> {
        if self.protected < count {
            return Err(Error::BadStack { location: count }.into());
        }
        self.protected -= count;
        Ok(())
    }

    pub fn push_at(&mut self, depth: usize, item: StackItem) -> Result<()> {
        let depth = depth + self.protected;
        trace_stack!("Insert", &item, Some(&depth));
        if self.items.len() < depth {
            return Err(Error::BadStack { location: depth }.into());
        }
        self.items.insert(depth, item);
        Ok(())
    }

    pub fn push(&mut self, item: StackItem) -> Result<()> {
        if self.protected > 0 {
            self.push_at(0, item)
        } else {
            trace_stack!("Push", &item, None);
            self.items.push_front(item);
            Ok(())
        }
    }

    pub fn pop_at(&mut self, depth: usize) -> Result<StackItem> {
        let depth = depth + self.protected;
        match self.items.remove(depth) {
            Some(item) => {
                trace_stack!("Remove", &item, Some(&depth));
                Ok(item)
            }
            None => Err(Error::BadStack { location: depth }.into()),
        }
    }

    pub fn pop(&mut self) -> Result<StackItem> {
        if self.protected > 0 {
            self.pop_at(0)
        } else {
            match self.items.pop_front() {
                Some(item) => {
                    trace_stack!("Pop", &item, None);
                    Ok(item)
                }
                None => Err(Error::BadStack { location: 0 }.into()),
            }
        }
    }

    pub fn dup_at(&self, depth: usize) -> Result<StackItem> {
        let depth = depth + self.protected;
        match self.items.get(depth) {
            Some(item) => {
                trace_stack!("Peek", &item, Some(&depth));
                Ok(item.clone())
            }
            None => Err(Error::BadStack { location: depth }.into()),
        }
    }
}
