use serde_json_wasm;
use tezos_michelson::micheline::{
    Micheline,
    sequence::Sequence,
    primitive_application::PrimitiveApplication
};
use tezos_michelson::michelson::{
    types::{Code, Type},
    data::Instruction
};
use proto::{
    Result,
    Error,
    vm::{StackItem, Stack, Interpreter, TransactionScope},
    context::ephemeral::EphemeralContext
};

pub struct TZT {
    inputs: Vec<StackItem>,
    outputs: Vec<StackItem>,
    code: Instruction
}

impl TZT {
    pub fn run(&self) -> Result<()> {
        let mut stack = Stack::new();
        let mut context = EphemeralContext::new();
        let scope = TransactionScope::default();

        for input in self.inputs.iter().rev() {
            stack.push(input.clone())?;
        }

        self.code.execute(&mut stack, &scope, &mut context)?;

        for output in self.outputs.iter() {
            let item = stack.pop()?;
            assert_eq!(*output, item);
        }

        Ok(())
    }
}

fn parse_stack_elts(section: PrimitiveApplication) -> Result<Vec<StackItem>> {
    let mut items: Vec<StackItem> = Vec::new();
    let outer_seq = Sequence::from(section.into_args().expect("outer sequence"));
    let inner_seq = outer_seq.into_values().remove(0).into_sequence().expect("inner sequence");

    for item in inner_seq.into_values() {
        let prim = PrimitiveApplication::try_from(item)?;
        match prim.prim() {
            "Stack_elt" => {
                assert_eq!(2, prim.args_count());
                let ty: Type = prim.nth_arg(0).expect("type").clone().try_into()?;
                let data = prim.nth_arg(1).expect("data").clone();
                items.push(StackItem::from_micheline(data, &ty)?);
            },
            _ => unreachable!()
        }
    }
    Ok(items)
}

impl TryFrom<Micheline> for TZT {
    type Error = Error;

    fn try_from(src: Micheline) -> Result<Self> {
        let mut inputs: Vec<StackItem> = Vec::new();
        let mut outputs: Vec<StackItem> = Vec::new();
        let mut code: Option<Instruction> = None;

        let sections = Sequence::try_from(src.normalized())?;
        for section in sections.into_values() {
            let prim = PrimitiveApplication::try_from(section)?;
            match prim.prim() {
                "input" => inputs = parse_stack_elts(prim)?,
                "output" => outputs = parse_stack_elts(prim)?,
                "code" => code = Some(*Code::try_from(prim)?.code),
                _ => unreachable!()
            }
        }

        Ok(Self { code: code.expect("code"), inputs, outputs})
    }
}

impl TryFrom<&str> for TZT {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let src: Micheline = serde_json_wasm::from_str(value)?;
        Self::try_from(src)
    }
}