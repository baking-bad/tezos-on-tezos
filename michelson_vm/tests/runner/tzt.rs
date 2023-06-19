use hex;
use tezos_core::{
    internal::crypto::blake2b,
    types::encoded::{self, Encoded, ScriptExprHash},
};
use tezos_michelson::micheline::{
    literals::Literal, primitive_application::PrimitiveApplication, sequence::Sequence, Micheline,
};
use tezos_michelson::michelson::{
    data::Instruction,
    types::{Code, Type},
};
use michelson_vm::typechecker::check_pair_len;
use michelson_vm::{
    interpreter::{Interpreter, InterpreterContext, LazyStorage, OperationScope},
    stack::Stack,
    trace_enter, trace_exit,
    types::big_map::get_key_hash,
    types::{BigMapDiff, BigMapItem, MapItem, StackItem},
    Error, Result,
};

use crate::runner::{
    micheline::read_from_file,
    mock::{default_scope, MockContext},
};

pub struct TZT {
    input: Input,
    output: Output,
    code: Instruction,
}

pub struct Input {
    pub items: Vec<StackItem>,
    pub scope: OperationScope,
    pub context: MockContext,
}

pub struct Output {
    pub items: Vec<StackItem>,
    pub error: Option<Error>,
}

fn compare_big_maps(lhs: MapItem, rhs: BigMapDiff, context: &mut MockContext) -> Result<()> {
    let ptr = rhs.value();
    let (elements, (kty, vty)) = lhs.into_elements();
    let count = elements.len();
    for (key, act) in elements {
        let key_hash = get_key_hash(&key, &kty)?;
        let exp = context
            .get_big_map_value(ptr, &key_hash)?
            .expect("Value is missing");
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

        match self
            .code
            .execute(&mut stack, &self.input.scope, &mut self.input.context)
        {
            Ok(()) => {
                for output in self.output.items.iter() {
                    let mut actual = stack.pop()?;
                    actual.try_acquire(&self.input.scope.self_address, &mut self.input.context)?;

                    let expected = output.clone();
                    match (expected, actual) {
                        (
                            StackItem::BigMap(BigMapItem::Map(lhs)),
                            StackItem::BigMap(BigMapItem::Diff(rhs)),
                        ) => {
                            compare_big_maps(lhs, rhs, &mut self.input.context)?;
                        }
                        (lhs, rhs) => assert_eq!(lhs, rhs),
                    }
                }
                trace_exit!();
            }
            Err(err) => {
                let expected = self.output.error.as_ref().expect("Error undefined");
                assert_eq!(*expected, err);
                trace_exit!(Some(&err.into()));
            }
        }

        Ok(())
    }

    pub fn load(filename: &str) -> Result<Self> {
        let src = read_from_file("tzt", filename)?;
        Self::try_from(src)
    }
}

fn parse_elements(sequence: Sequence) -> Result<Vec<StackItem>> {
    let mut items: Vec<StackItem> = Vec::new();
    for item in sequence.into_values() {
        let prim = PrimitiveApplication::try_from(item)?;
        match prim.prim() {
            "Stack_elt" => {
                check_pair_len(prim.args_count())?;
                let ty: Type = prim.nth_arg(0).expect("type").clone().try_into()?;
                let data = prim.nth_arg(1).expect("data").clone();
                items.push(StackItem::from_micheline(data, &ty)?);
            }
            _ => panic!("Expected `Stack_elt`"),
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
                        with: arg.first_arg().expect("Expected `with` arg").clone(),
                    })
                }
                _ => panic!("Unknown primitive in output args"),
            },
            Micheline::Sequence(seq) => items = parse_elements(seq)?,
            _ => panic!("Unexpected output arg"),
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
        _ => panic!("Expected literal"),
    }
}

fn parse_input(section: PrimitiveApplication) -> Result<Vec<StackItem>> {
    for arg in section.into_args().expect("Expected single arg") {
        match arg {
            Micheline::Sequence(seq) => return parse_elements(seq),
            _ => panic!("Expected input sequence"),
        }
    }
    panic!("Input section is empty")
}

fn parse_contracts(sequence: Sequence, context: &mut MockContext) -> Result<()> {
    for item in sequence.into_values() {
        let prim = PrimitiveApplication::try_from(item)?;
        match prim.prim() {
            "Contract" => {
                let mut args = prim.into_args().expect("Contract args missing");
                assert_eq!(2, args.len());
                let key = args
                    .remove(0)
                    .into_literal()
                    .expect("Expected literal")
                    .into_micheline_string()
                    .expect("Expected string")
                    .into_string()
                    .try_into()?;
                context.set_contract_type(key, args.remove(0))?;
            }
            _ => panic!("Expected `Contract`"),
        }
    }
    Ok(())
}

fn parse_other(section: PrimitiveApplication, context: &mut MockContext) -> Result<()> {
    for arg in section.into_args().expect("Expected single arg") {
        match arg {
            Micheline::Sequence(seq) => return parse_contracts(seq, context),
            _ => panic!("Expected other sequence"),
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

fn parse_big_map_values(
    sequence: Sequence,
    scope: &OperationScope,
    context: &mut MockContext,
) -> Result<()> {
    for item in sequence.into_values() {
        let prim = PrimitiveApplication::try_from(item)?;
        match prim.prim() {
            "Big_map" => {
                let mut args = prim.into_args().expect("Contract args missing");
                assert_eq!(4, args.len());
                let ptr: i64 = args
                    .remove(0)
                    .into_literal()
                    .expect("Expected literal")
                    .into_micheline_int()
                    .expect("Expected int")
                    .try_into()?;
                context.init_big_map(ptr, scope.self_address.clone());
                let schema = args.remove(0);
                let elts: Vec<PrimitiveApplication> = args
                    .remove(1)
                    .into_sequence()
                    .expect("Expected sequence")
                    .into_values()
                    .into_iter()
                    .map(|elt| elt.into_primitive_application().expect("Expected `Elt`"))
                    .collect();
                for elt in elts {
                    let mut args = elt.into_args().expect("Elt args missing");
                    assert_eq!(2, args.len());
                    let key = args.remove(0);
                    let value = args.remove(0);
                    let key_hash = script_expr_hash(key, &schema)?;
                    context.set_big_map_value(ptr, key_hash, Some(value))?;
                }
            }
            _ => panic!("Expected `Big_map`"),
        }
    }
    Ok(())
}

fn parse_big_maps(
    section: PrimitiveApplication,
    scope: &OperationScope,
    context: &mut MockContext,
) -> Result<()> {
    for arg in section.into_args().expect("Expected single arg") {
        match arg {
            Micheline::Sequence(seq) => return parse_big_map_values(seq, scope, context),
            _ => panic!("Expected other sequence"),
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
                "balance" => scope.balance = parse_literal(prim).try_into()?,
                "sender" => scope.sender = parse_literal(prim).try_into()?,
                "source" => scope.source = parse_literal(prim).try_into()?,
                "now" => {
                    scope.now = i64::from_str_radix(parse_literal(prim).as_str(), 10).expect("ts")
                }
                "self" => scope.self_address = parse_literal(prim).try_into()?,
                "parameter" => scope.self_type = prim.into(),
                "chain_id" => {
                    let bytes = hex::decode(parse_literal(prim)).expect("Chain ID");
                    scope.chain_id = encoded::ChainId::from_bytes(bytes.as_slice())?;
                }
                "other_contracts" => parse_other(prim, &mut context)?,
                "big_maps" => parse_big_maps(prim, &scope, &mut context)?,
                prim => panic!("Unexpected section {}", prim),
            }
        }

        Ok(Self {
            code: code.expect("Failed to parse code"),
            output: output.expect("Failed to parse output"),
            input: Input {
                items,
                scope,
                context,
            },
        })
    }
}
