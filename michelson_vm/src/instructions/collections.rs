// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use tezos_michelson::michelson::data::instructions::{
    Cons, EmptyBigMap, EmptyMap, EmptySet, Get, GetAndUpdate, Mem, Nil, Update,
};

use crate::{
    err_mismatch,
    interpreter::{
        ContextInterpreter, Interpreter, InterpreterContext, OperationScope, PureInterpreter,
    },
    pop_cast,
    stack::Stack,
    types::{BigMapDiff, BigMapItem, ListItem, MapItem, SetItem, StackItem},
    Result,
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
    fn execute(
        &self,
        stack: &mut Stack,
        scope: &OperationScope,
        context: &mut impl InterpreterContext,
    ) -> Result<()> {
        let ptr = context.allocate_big_map(scope.self_address.clone())?;
        let big_map = BigMapItem::Diff(BigMapDiff::new(
            ptr,
            self.key_type.clone(),
            self.value_type.clone(),
        ));
        stack.push(big_map.into())
    }
}

impl ContextInterpreter for Mem {
    fn execute(&self, stack: &mut Stack, context: &mut impl InterpreterContext) -> Result<()> {
        let key = stack.pop()?;
        let res = match stack.pop()? {
            StackItem::Set(set) => set.contains(&key)?,
            StackItem::Map(map) => map.contains(&key)?,
            StackItem::BigMap(big_map) => big_map.contains(&key, context)?,
            item => return err_mismatch!("SetItem, MapItem, or BigMapItem", item),
        };
        stack.push(StackItem::Bool(res.into()))
    }
}

impl ContextInterpreter for Get {
    fn execute(&self, stack: &mut Stack, context: &mut impl InterpreterContext) -> Result<()> {
        let res = if let Some(n) = &self.n {
            let pair = pop_cast!(stack, Pair);
            let idx: usize = n.try_into()?;
            pair.get(idx)?
        } else {
            let key = stack.pop()?;
            match stack.pop()? {
                StackItem::Map(map) => map.get(&key)?.into(),
                StackItem::BigMap(big_map) => big_map.get(&key, context)?.into(),
                item => return err_mismatch!("MapItem or BigMapItem", item),
            }
        };
        stack.push(res)
    }
}

impl Interpreter for Update {
    fn execute(
        &self,
        stack: &mut Stack,
        scope: &OperationScope,
        context: &mut impl InterpreterContext,
    ) -> Result<()> {
        let res: StackItem = if let Some(n) = &self.n {
            let item = stack.pop()?;
            let idx = n.try_into()?;
            let mut pair = pop_cast!(stack, Pair);
            pair.update(idx, item)?;
            pair.into()
        } else {
            let key = stack.pop()?;
            match stack.pop()? {
                StackItem::Bool(val) => {
                    let mut set = pop_cast!(stack, Set);
                    set.update(key, val.is_true())?;
                    set.into()
                }
                StackItem::Option(val) => match stack.pop()? {
                    StackItem::Map(mut map) => {
                        map.update(key, val.unwrap())?;
                        map.into()
                    }
                    StackItem::BigMap(big_map) => {
                        let mut big_map = big_map.acquire(&scope.self_address, context)?;
                        big_map.update(key, val.unwrap(), context)?;
                        big_map.into()
                    }
                    item => return err_mismatch!("MapItem or BigMapItem", item),
                },
                item => return err_mismatch!("BoolItem or OptionItem", item),
            }
        };
        stack.push(res)
    }
}

impl Interpreter for GetAndUpdate {
    fn execute(
        &self,
        stack: &mut Stack,
        scope: &OperationScope,
        context: &mut impl InterpreterContext,
    ) -> Result<()> {
        let key = stack.pop()?;
        let val = pop_cast!(stack, Option);
        match stack.pop()? {
            StackItem::Map(mut map) => {
                let old = map.update(key, val.unwrap())?;
                stack.push(map.into())?;
                stack.push(old.into())
            }
            StackItem::BigMap(big_map) => {
                let mut big_map = big_map.acquire(&scope.self_address, context)?;
                let old = big_map.get(&key, context)?;
                big_map.update(key, val.unwrap(), context)?;
                stack.push(big_map.into())?;
                stack.push(old.into())
            }
            item => return err_mismatch!("MapItem or BigMapItem", item),
        }
    }
}
