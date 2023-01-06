
use serde_json_wasm;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::collections::HashMap;
use hex;
use tezos_michelson::micheline::{
    Micheline,
    sequence::Sequence,
    primitive_application::PrimitiveApplication,
    literals::Literal
};
use tezos_michelson::michelson::{
    types::{Code, Type},
    data::Instruction
};
use tezos_core::{
    types::encoded::{self, ScriptExprHash, Encoded},
    internal::crypto::blake2b
};
use vm::{
    Result,
    Error,
    stack::Stack,
    types::{StackItem, BigMapItem, MapItem, BigMapPtr},
    types::big_map::get_key_hash,
    interpreter::{Interpreter, LazyStorage, TransactionScope, TransactionContext},
    trace_enter,
    trace_exit
};
use crate::runner::mock::{default_scope, MockContext};

pub struct TZT {
    input: Input,
    output: Output,
    code: Instruction,
}

pub struct Input {
    pub items: Vec<StackItem>,
    pub scope: TransactionScope,
    pub context: MockContext
}

pub struct Output {
    pub items: Vec<StackItem>,
    pub error: Option<Error>
}

fn compare_big_maps(lhs: MapItem, rhs: BigMapPtr, context: &MockContext) -> Result<()> {
    let ptr = rhs.value();
    let (elements, (kty, vty)) = lhs.into_elements();
    let count = elements.len();
    for (key, act) in elements {
        let key_hash = get_key_hash(&key, &kty)?;
        let exp = context.get_big_map_value(ptr, &key_hash)?.expect("Value is missing");
        assert_eq!(exp, act.into_micheline(&vty)?);
    }
    assert_eq!(count, context.get_elements_count(ptr));
    Ok(())
}

impl TZT {
    pub fn run(&mut self) -> Result<()> {
        let mut stack = Stack::new();
        trace_enter!();

        for input in self.input.items.iter().rev() {
            stack.push(input.clone())?;
        }

        match self.code.execute(&mut stack, &self.input.scope, &mut self.input.context) {
            Ok(()) => {
                for output in self.output.items.iter() {
                    let expected = output.clone();
                    let mut actual = stack.pop()?;
                    actual.try_acquire(&self.input.scope, &mut self.input.context)?;
                    match (expected, actual) {
                        (StackItem::BigMap(BigMapItem::Map(lhs)), StackItem::BigMap(BigMapItem::Ptr(rhs))) => {
                            compare_big_maps(lhs, rhs, &self.input.context)?;
                        },
                        (lhs, rhs) => assert_eq!(lhs, rhs)
                    }
                }
                trace_exit!();
            },
            Err(err) => {
                let expected = self.output.error.as_ref().expect("Error undefined");
                assert_eq!(*expected, err);
                trace_exit!(Some(&err.into()));
            }
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
            _ => panic!("Expected `Stack_elt`")
        }
    }
    Ok(items)
}

fn parse_output(section: PrimitiveApplication) -> Result<Output> {
    let mut items: Vec<StackItem> = Vec::new();
    let mut error: Option<Error> = None;
    for arg in section.into_args().expect("Expected single arg") {
        match arg {
            Micheline::PrimitiveApplication(arg) => match arg.prim() {
                "MutezOverflow" => error = Some(Error::MutezOverflow),
                "MutezUnderflow" => error = Some(Error::MutezUnderflow),
                "GeneralOverflow" => error = Some(Error::GeneralOverflow),
                "Failed" => {
                    error = Some(Error::ScriptFailed {
                        with: arg.first_arg().expect("Expected `with` arg").clone()
                    })
                },
                _ => panic!("Unknown primitive in output args")
            },
            Micheline::Sequence(seq) => items = parse_elements(seq)?,
            _ => panic!("Unexpected output arg")
        }
    }
    Ok(Output { items, error })
}

fn parse_literal(outer: PrimitiveApplication) -> String {
    let mut args = outer.into_args().expect("Expected single arg");
    match args.remove(0) {
        Micheline::Literal(Literal::Int(int)) => int.to_string(),
        Micheline::Literal(Literal::String(string)) => string.into_string(),
        Micheline::Literal(Literal::Bytes(bytes)) => bytes.value()[2..].to_string(),
        _ => panic!("Expected literal")
    }
}

fn parse_input(section: PrimitiveApplication) -> Result<Vec<StackItem>> {
    for arg in section.into_args().expect("Expected single arg") { 
        match arg {
            Micheline::Sequence(seq) => return parse_elements(seq),
            _ => panic!("Expected input sequence")
        }
    }
    panic!("Input section is empty")
}

fn parse_contracts(sequence: Sequence) -> Result<HashMap<String, Micheline>> {
    let mut contracts: HashMap<String, Micheline> = HashMap::new();
    for item in sequence.into_values() {
        let prim = PrimitiveApplication::try_from(item)?;
        match prim.prim() {
            "Contract" => {
                let mut args = prim.into_args().expect("Contract args missing");
                assert_eq!(2, args.len());
                let key = args.remove(0)
                    .into_literal().expect("Expected literal")
                    .into_micheline_string().expect("Expected string")
                    .into_string();
                contracts.insert(key, args.remove(0));
            },
            _ => panic!("Expected `Contract`")
        }
    }
    Ok(contracts)
}

fn parse_other(section: PrimitiveApplication) -> Result<HashMap<String, Micheline>> {
    for arg in section.into_args().expect("Expected single arg") { 
        match arg {
            Micheline::Sequence(seq) => return parse_contracts(seq),
            _ => panic!("Expected other sequence")
        }
    }
    panic!("Other section is empty")
}

pub fn script_expr_hash(data: Micheline, schema: &Micheline) -> Result<ScriptExprHash> {
    let payload = data.pack(Some(&schema))?;
    let hash = blake2b(payload.as_slice(), 32)?;
    let res = ScriptExprHash::from_bytes(hash.as_slice())?;
    Ok(res)
}

fn parse_big_map_values(sequence: Sequence, scope: &TransactionScope, context: &mut MockContext) -> Result<()> {
    for item in sequence.into_values() {
        let prim = PrimitiveApplication::try_from(item)?;
        match prim.prim() {
            "Big_map" => {
                let mut args = prim.into_args().expect("Contract args missing");
                assert_eq!(4, args.len());
                let ptr: i64 = args.remove(0)
                    .into_literal().expect("Expected literal")
                    .into_micheline_int().expect("Expected int")
                    .to_integer()?;
                context.init_big_map(ptr, scope.self_address.clone());
                let schema = args.remove(0);
                let elts: Vec<PrimitiveApplication> = args.remove(1)
                    .into_sequence().expect("Expected sequence")
                    .into_values()
                    .into_iter().map(|elt| elt.into_primitive_application().expect("Expected `Elt`"))
                    .collect();
                for elt in elts {
                    let mut args = elt.into_args().expect("Elt args missing");
                    assert_eq!(2, args.len());
                    let key = args.remove(0);
                    let value = args.remove(0);
                    let key_hash = script_expr_hash(key, &schema)?;
                    context.set_big_map_value(ptr, key_hash, Some(value))?;
                }
            },
            _ => panic!("Expected `Big_map`")
        }
    }
    Ok(())
}

fn parse_big_maps(section: PrimitiveApplication, scope: &TransactionScope, context: &mut MockContext) -> Result<()> {
    for arg in section.into_args().expect("Expected single arg") { 
        match arg {
            Micheline::Sequence(seq) => return parse_big_map_values(seq, scope, context),
            _ => panic!("Expected other sequence")
        }
    }
    panic!("Bigmaps section is empty")
}

impl TryFrom<Micheline> for TZT {
    type Error = Error;

    fn try_from(src: Micheline) -> Result<Self> {
        let mut items: Vec<StackItem> = Vec::new();
        let mut scope = default_scope();
        let mut context = MockContext::default();
        let mut output: Option<Output> = None;
        let mut code: Option<Instruction> = None;

        let sections = Sequence::try_from(src.normalized())?;
        for section in sections.into_values() {
            let prim = PrimitiveApplication::try_from(section)?;
            match prim.prim() {
                "input" => items = parse_input(prim)?,
                "output" => output = Some(parse_output(prim)?),
                "code" => code = Some(*Code::try_from(prim)?.code),
                "amount" => scope.amount = parse_literal(prim).try_into()?,
                "sender" => scope.sender = parse_literal(prim).try_into()?,
                "source" => scope.source = parse_literal(prim).try_into()?,
                "chain_id" => {
                    let bytes = hex::decode(parse_literal(prim)).expect("Chain ID");
                    scope.chain_id = encoded::ChainId::from_bytes(bytes.as_slice())?;
                },
                "now" => scope.now = i64::from_str_radix(parse_literal(prim).as_str(), 10).expect("ts"),
                "balance" => context.balance = parse_literal(prim).try_into()?,
                "other_contracts" => context.contracts = parse_other(prim)?,
                "big_maps" => parse_big_maps(prim, &scope, &mut context)?,
                "self" => scope.self_address = parse_literal(prim).try_into()?,
                "parameter" => {
                    context.contracts.insert(scope.self_address.into_string(), prim.into_args().unwrap().remove(0));
                },
                prim => panic!("Unexpected section {}", prim)
            }
        }

        Ok(Self {
            code: code.expect("Failed to parse code"),
            output: output.expect("Failed to parse output"),
            input: Input { items, scope, context },
        })
    }
}
