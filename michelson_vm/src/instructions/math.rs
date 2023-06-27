// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use tezos_michelson::michelson::data::instructions::{
    Abs, Add, And, Ediv, Int, IsNat, Lsl, Lsr, Mul, Neg, Not, Or, Sub, SubMutez, Xor,
};
use tezos_michelson::michelson::types;

use crate::{
    err_mismatch,
    interpreter::PureInterpreter,
    pop_cast,
    stack::Stack,
    types::{OptionItem, StackItem},
    Error, Result,
};

macro_rules! invalid_operands {
    ($opcode: literal, $l: expr, $r: expr) => {
        err_mismatch!(
            format!("{}-supported operands", $opcode),
            format!("{} * {}", $l, $r)
        )
    };
}

impl PureInterpreter for Abs {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let int = pop_cast!(stack, Int);
        stack.push(int.abs()?.into())
    }
}

impl PureInterpreter for Add {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let a = stack.pop()?;
        let b = stack.pop()?;
        let res: StackItem = match (a, b) {
            (StackItem::Nat(a), StackItem::Nat(b)) => (a + b).into(),
            (StackItem::Nat(a), StackItem::Int(b)) => (a + b).into(),
            (StackItem::Int(a), StackItem::Nat(b)) => (b + a).into(),
            (StackItem::Int(a), StackItem::Int(b)) => (a + b).into(),
            (StackItem::Timestamp(a), StackItem::Int(b)) => (a + b)?.into(),
            (StackItem::Int(a), StackItem::Timestamp(b)) => (b + a)?.into(),
            (StackItem::Mutez(a), StackItem::Mutez(b)) => (a + b)?.into(),
            (l, r) => return invalid_operands!("ADD", l, r),
        };
        stack.push(res)
    }
}

impl PureInterpreter for Ediv {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let a = stack.pop()?;
        let b = stack.pop()?;
        let res: StackItem = match (a, b) {
            (StackItem::Nat(a), StackItem::Nat(b)) => (a / b).into(),
            (StackItem::Nat(a), StackItem::Int(b)) => (a.int() / b).into(),
            (StackItem::Int(a), StackItem::Nat(b)) => (a / b.int()).into(),
            (StackItem::Int(a), StackItem::Int(b)) => (a / b).into(),
            (StackItem::Mutez(a), StackItem::Nat(b)) => (a / b)?.into(),
            (StackItem::Mutez(a), StackItem::Mutez(b)) => (a / b)?.into(),
            (l, r) => return invalid_operands!("EDIV", l, r),
        };
        stack.push(res)
    }
}

impl PureInterpreter for Lsl {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let a = pop_cast!(stack, Nat);
        let b = pop_cast!(stack, Nat);
        let res = (a << b)?;
        stack.push(res.into())
    }
}

impl PureInterpreter for Lsr {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let a = pop_cast!(stack, Nat);
        let b = pop_cast!(stack, Nat);
        let res = (a >> b)?;
        stack.push(res.into())
    }
}

impl PureInterpreter for Mul {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let a = stack.pop()?;
        let b = stack.pop()?;
        let res: StackItem = match (a, b) {
            (StackItem::Nat(a), StackItem::Nat(b)) => (a * b).into(),
            (StackItem::Nat(a), StackItem::Int(b)) => (a.int() * b).into(),
            (StackItem::Int(a), StackItem::Nat(b)) => (a * b.int()).into(),
            (StackItem::Int(a), StackItem::Int(b)) => (a * b).into(),
            (StackItem::Mutez(a), StackItem::Nat(b)) => (a * b)?.into(),
            (StackItem::Nat(a), StackItem::Mutez(b)) => (b * a)?.into(),
            (l, r) => return invalid_operands!("MUL", l, r),
        };
        stack.push(res)
    }
}

impl PureInterpreter for Neg {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let res: StackItem = match stack.pop()? {
            StackItem::Nat(a) => (-a).into(),
            StackItem::Int(a) => (-a).into(),
            items => return err_mismatch!("NatItem or IntItem", items),
        };
        stack.push(res)
    }
}

impl PureInterpreter for Sub {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let a = stack.pop()?;
        let b = stack.pop()?;
        let res: StackItem = match (a, b) {
            (StackItem::Nat(a), StackItem::Nat(b)) => (a - b).into(),
            (StackItem::Nat(a), StackItem::Int(b)) => (a - b).into(),
            (StackItem::Int(a), StackItem::Nat(b)) => (a - b).into(),
            (StackItem::Int(a), StackItem::Int(b)) => (a - b).into(),
            (StackItem::Timestamp(a), StackItem::Int(b)) => (a - b)?.into(),
            (StackItem::Timestamp(a), StackItem::Timestamp(b)) => (a - b).into(),
            (StackItem::Mutez(a), StackItem::Mutez(b)) => (a - b)?.into(),
            (l, r) => return invalid_operands!("SUB", l, r),
        };
        stack.push(res)
    }
}

impl PureInterpreter for SubMutez {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let a = pop_cast!(stack, Mutez);
        let b = pop_cast!(stack, Mutez);
        let res = match a - b {
            Ok(res) => OptionItem::some(res.into()),
            Err(Error::MutezUnderflow) => OptionItem::none(&types::mutez()),
            Err(err) => return Err(err),
        };
        stack.push(res.into())
    }
}

impl PureInterpreter for Int {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let nat = pop_cast!(stack, Nat);
        stack.push(nat.int().into())
    }
}

impl PureInterpreter for IsNat {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let int = pop_cast!(stack, Int);
        stack.push(int.nat()?.into())
    }
}

impl PureInterpreter for Or {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let a = stack.pop()?;
        let b = stack.pop()?;
        let res: StackItem = match (a, b) {
            (StackItem::Bool(a), StackItem::Bool(b)) => (a | b).into(),
            (StackItem::Nat(a), StackItem::Nat(b)) => (a | b).into(),
            (l, r) => return invalid_operands!("OR", l, r),
        };
        stack.push(res)
    }
}

impl PureInterpreter for Xor {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let a = stack.pop()?;
        let b = stack.pop()?;
        let res: StackItem = match (a, b) {
            (StackItem::Bool(a), StackItem::Bool(b)) => (a ^ b).into(),
            (StackItem::Nat(a), StackItem::Nat(b)) => (a ^ b).into(),
            (l, r) => return invalid_operands!("XOR", l, r),
        };
        stack.push(res)
    }
}

impl PureInterpreter for And {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let a = stack.pop()?;
        let b = stack.pop()?;
        let res: StackItem = match (a, b) {
            (StackItem::Bool(a), StackItem::Bool(b)) => (a & b).into(),
            (StackItem::Nat(a), StackItem::Nat(b)) => (a & b).into(),
            (StackItem::Int(a), StackItem::Nat(b)) => (a & b)?.into(),
            (l, r) => return invalid_operands!("AND", l, r),
        };
        stack.push(res)
    }
}

impl PureInterpreter for Not {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let res: StackItem = match stack.pop()? {
            StackItem::Bool(a) => (!a).into(),
            StackItem::Nat(a) => (!a).into(),
            StackItem::Int(a) => (!a).into(),
            item => return err_mismatch!("NOT-supported operand", item),
        };
        stack.push(res)
    }
}
