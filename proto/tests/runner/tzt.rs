use serde_json_wasm;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
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
    error::InterpreterError,
    vm::{StackItem, Stack, Interpreter, TransactionScope, trace_init, trace_ret},
    context::ephemeral::EphemeralContext,
};

pub struct TZT {
    input: Input,
    output: Output,
    code: Instruction,
}

pub struct Input {
    pub items: Vec<StackItem>,
    pub scope: TransactionScope
}

pub struct Output {
    pub items: Vec<StackItem>,
    pub error: Option<InterpreterError>
}

impl TZT {
    pub fn run(&self) -> Result<()> {
        let mut stack = Stack::new();
        let mut context = EphemeralContext::new();
        let scope = TransactionScope::default();
        trace_init("tzt");

        for input in self.input.items.iter().rev() {
            stack.push(input.clone())?;
        }

        let res = self.code.execute(&mut stack, &scope, &mut context);

        match res {
            Ok(()) => {
                for output in self.output.items.iter() {
                    let item = stack.pop()?;
                    assert_eq!(*output, item);
                }
                trace_ret(None);
            },
            Err(Error::InterpreterError(err)) => {
                let expected = self.output.error.as_ref().expect("Error undefined");
                assert_eq!(*expected, err);
                trace_ret(Some(&err.into()));
            },
            _ => unreachable!("Unexpected result")
        }

        Ok(())
    }

    pub fn load(filename: &str) -> Result<Self> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/data");
        path.push(filename);

        let mut file = File::open(path).expect("Failed to open tzt file");
        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer).expect("Failed to read tzt file");
        
        let src: Micheline = serde_json_wasm::from_slice(buffer.as_slice())?;
        Self::try_from(src)
    }
}

fn parse_elements(sequence: Sequence) -> Result<Vec<StackItem>> {
    let mut items: Vec<StackItem> = Vec::new();
    for item in sequence.into_values() {
        let prim = PrimitiveApplication::try_from(item)?;
        match prim.prim() {
            "Stack_elt" => {
                assert_eq!(2, prim.args_count());
                let ty: Type = prim.nth_arg(0).expect("type").clone().try_into()?;
                let data = prim.nth_arg(1).expect("data").clone();
                items.push(StackItem::from_micheline(data, &ty)?);
            },
            _ => unreachable!("Expected `Stack_elt`")
        }
    }
    Ok(items)
}

fn parse_output(section: PrimitiveApplication) -> Result<Output> {
    let mut items: Vec<StackItem> = Vec::new();
    let mut error: Option<InterpreterError> = None;
    for arg in section.into_args().expect("Expected single arg") {
        match arg {
            Micheline::PrimitiveApplication(arg) => match arg.prim() {
                "MutezOverflow" => error = Some(InterpreterError::MutezOverflow),
                "MutezUnderflow" => error = Some(InterpreterError::MutezUnderflow),
                "GeneralOverflow" => error = Some(InterpreterError::GeneralOverflow),
                "Failed" => {
                    error = Some(InterpreterError::ScriptFailed {
                        with: arg.first_arg().expect("Expected `with` arg").clone()
                    })
                },
                _ => unreachable!("Unknown primitive in output args")
            },
            Micheline::Sequence(seq) => items = parse_elements(seq)?,
            _ => unreachable!("Unexpected output arg")
        }
    }
    Ok(Output { items, error })
}

fn parse_input(section: PrimitiveApplication) -> Result<Vec<StackItem>> {
    for arg in section.into_args().expect("Expected single arg") { 
        match arg {
            Micheline::Sequence(seq) => return parse_elements(seq),
            _ => unreachable!("Input section has to have inner sequence as arg")
        }
    }
    unreachable!("Input section has no args")
}

impl TryFrom<Micheline> for TZT {
    type Error = Error;

    fn try_from(src: Micheline) -> Result<Self> {
        let mut scope = TransactionScope::default();
        let mut inputs: Vec<StackItem> = Vec::new();
        let mut output: Option<Output> = None;
        let mut code: Option<Instruction> = None;

        let sections = Sequence::try_from(src.normalized())?;
        for section in sections.into_values() {
            let prim = PrimitiveApplication::try_from(section)?;
            match prim.prim() {
                "input" => inputs = parse_input(prim)?,
                "output" => output = Some(parse_output(prim)?),
                "code" => code = Some(*Code::try_from(prim)?.code),
                _ => unreachable!()
            }
        }

        Ok(Self {
            code: code.expect("Failed to parse code"),
            output: output.expect("Failed to parse output"),
            input: Input {items: inputs, scope}
        })
    }
}
