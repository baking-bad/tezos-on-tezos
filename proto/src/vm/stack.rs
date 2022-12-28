use std::collections::VecDeque;

use crate::{
    error::{Result, Error}
};
pub use crate::vm::types::StackItem;

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
            return Err(Error::StackOutOfBounds)
        }
        self.protected += count;
        Ok(())
    }

    pub fn restore(&mut self, count: usize) -> Result<()> {
        if self.protected < count {
            return Err(Error::StackOutOfBounds)
        }
        self.protected -= count;
        Ok(())
    }

    pub fn push_at(&mut self, depth: usize, item: StackItem) -> Result<()> {
        if self.items.len() < depth {
            return Err(Error::StackOutOfBounds)
        }
        self.items.insert(depth, item);
        Ok(())
    }

    pub fn push(&mut self, item: StackItem) -> Result<()> {
        if self.protected > 0 {
            self.push_at(self.protected, item)
        } else {
            self.items.push_front(item);
            Ok(())
        }
    }

    pub fn pop_at(&mut self, depth: usize) -> Result<StackItem> {
        match self.items.remove(depth) {
            Some(item) => Ok(item),
            None => Err(Error::StackOutOfBounds)
        }
    }

    pub fn pop(&mut self) -> Result<StackItem> {
        if self.protected > 0 {
            self.pop_at(self.protected)
        } else {
            match self.items.pop_front() {
                Some(item) => Ok(item),
                None => Err(Error::StackOutOfBounds)
            }
        }
    }

    pub fn dup_at(&self, depth: usize) -> Result<StackItem> {
        match self.items.get(depth) {
            Some(item) => Ok(item.clone()),
            None => Err(Error::StackOutOfBounds)
        }
    }
}
