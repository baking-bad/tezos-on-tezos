use tezos_michelson::michelson::{
    data::instructions::{Nil, Cons, EmptySet, EmptyMap, EmptyBigMap, Mem, Get, Update, GetAndUpdate}
};
use tezos_core::types::encoded::Address;

use crate::{
    Result,
    interpreter::{Interpreter, PureInterpreter, ContextIntepreter, TransactionContext, TransactionScope},
    types::{StackItem, ListItem, SetItem, MapItem, BigMapItem, BigMapPtr},
    stack::Stack,
    pop_cast,
    err_type
};

impl PureInterpreter for Nil {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let list = ListItem::new(vec![], self.r#type.clone());
        stack.push(list.into())
    }
}

impl PureInterpreter for Cons {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let item = stack.pop()?;
        let mut list = pop_cast!(stack, List);
        list.prepend(item)?;
        stack.push(list.into())
    }
}

impl PureInterpreter for EmptySet {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let set = SetItem::new(vec![], self.r#type.clone());
        stack.push(set.into())
    }
}

impl PureInterpreter for EmptyMap {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let map = MapItem::new(vec![], self.key_type.clone(), self.value_type.clone());
        stack.push(map.into())
    }
}

impl Interpreter for EmptyBigMap {
    fn execute(&self, stack: &mut Stack, scope: &TransactionScope, context: &mut impl TransactionContext) -> Result<()> {
        let ptr = context.allocate_big_map(Address::Originated(scope.self_address.clone()))?;
        let big_map = BigMapItem::Ptr(
            BigMapPtr::new(ptr, self.key_type.clone(), self.value_type.clone())
        );
        stack.push(big_map.into())
    }
}

impl ContextIntepreter for Mem {
    fn execute(&self, stack: &mut Stack, context: &mut impl TransactionContext) -> Result<()> {
        let key = stack.pop()?;
        let res = match stack.pop()? {
            StackItem::Set(set) => set.contains(&key)?,
            StackItem::Map(map) => map.contains(&key)?,
            StackItem::BigMap(big_map) => big_map.contains(&key, context)?,
            item => return err_type!("SetItem, MapItem, or BigMapItem", item)
        };
        stack.push(StackItem::Bool(res.into()))
    }
}

impl ContextIntepreter for Get {
    fn execute(&self, stack: &mut Stack, context: &mut impl TransactionContext) -> Result<()> {
        let res = if let Some(n) = &self.n {
            let pair = pop_cast!(stack, Pair);
            let idx: usize = n.to_integer()?;
            pair.get(idx)?
        } else {
            let key = stack.pop()?;
            match stack.pop()? {
                StackItem::Map(map) => map.get(&key)?.into(),
                StackItem::BigMap(big_map) => big_map.get(&key, context)?.into(),
                item => return err_type!("MapItem or BigMapItem", item)
            }
        };
        stack.push(res)
    }
}

impl ContextIntepreter for Update {
    fn execute(&self, stack: &mut Stack, context: &mut impl TransactionContext) -> Result<()> {
        let res: StackItem = if let Some(n) = &self.n {
            let item = stack.pop()?;
            let idx = n.to_integer()?;
            let pair = pop_cast!(stack, Pair);
            pair.update(idx, item)?.into()
        } else {
            let key = stack.pop()?;
            match stack.pop()? {
                StackItem::Bool(val) => {
                    let mut set = pop_cast!(stack, Set);
                    set.update(key, val.is_true())?;
                    set.into()
                },
                StackItem::Option(val) => match stack.pop()? {
                    StackItem::Map(mut map) => {
                        map.update(key, val.unwrap())?;
                        map.into()
                    },
                    StackItem::BigMap(mut big_map) => {
                        big_map.update(key, val.unwrap(), context)?;
                        big_map.into()
                    },
                    item => return err_type!("MapItem or BigMapItem", item)
                },
                item => return err_type!("BoolItem or OptionItem", item)
            }
        };
        stack.push(res)
    }
}

impl ContextIntepreter for GetAndUpdate {
    fn execute(&self, stack: &mut Stack, context: &mut impl TransactionContext) -> Result<()> {
        let key = stack.pop()?;
        let val = pop_cast!(stack, Option);
        match stack.pop()? {
            StackItem::Map(mut map) => {
                let old = map.update(key, val.unwrap())?;
                stack.push(map.into())?;
                stack.push(old.into())
            },
            StackItem::BigMap(mut big_map) => {
                let old = big_map.update(key, val.unwrap(), context)?;
                stack.push(big_map.into())?;
                stack.push(old.into())
            },
            item => return err_type!("MapItem or BigMapItem", item)
        }
    }
}