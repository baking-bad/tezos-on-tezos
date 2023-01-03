use tezos_michelson::michelson::{
    data::instructions::{Nil, Cons, EmptySet, EmptyMap, Mem, Get, Update, GetAndUpdate}
};

use crate::{
    Result,
    interpreter::{PureInterpreter, ContextIntepreter, TransactionContext},
    types::{StackItem, ListItem, SetItem, MapItem},
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
        let list = pop_cast!(stack, List);
        let res = list.prepend(item)?;
        stack.push(res.into())
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

impl ContextIntepreter for Mem {
    fn execute(&self, stack: &mut Stack, context: &mut impl TransactionContext) -> Result<()> {
        let key = stack.pop()?;
        let res = match stack.pop()? {
            StackItem::Set(set) => set.contains(&key)?,
            StackItem::Map(map) => map.contains(&key)?,
            StackItem::BigMap(big_map) => todo!("Context bindings"),
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
                StackItem::BigMap(big_map) => todo!("Context bindings"),
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
                    let set = pop_cast!(stack, Set);
                    set.update(key, val.is_true())?.into()
                },
                StackItem::Option(val) => match stack.pop()? {
                    StackItem::Map(map) => {
                        let (res, _) = map.update(key, val.unwrap())?;
                        res.into()
                    },
                    StackItem::BigMap(big_map) => todo!("Context bindings"),
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
            StackItem::Map(map) => {
                let (res, old) = map.update(key, val.unwrap())?;
                stack.push(res.into())?;
                stack.push(old.into())
            },
            StackItem::BigMap(big_map) => todo!(),
            item => return err_type!("MapItem or BigMapItem", item)
        }
    }
}