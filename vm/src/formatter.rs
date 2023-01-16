use tezos_michelson::michelson::{
    types::{Type, ComparableType},
    data::{Data, Instruction},
    annotations::{Kind, Annotation}
};
use tezos_core::types::number::Nat;

pub trait Formatter {
    fn format(&self) -> String;
}


fn format_instr_n(opcode: &str, arg: Option<&Nat>) -> String {
    match arg {
        Some(arg) => format!("{} {}", opcode, arg.to_str()),
        None => opcode.to_string(),
    }
}

fn format_instr_annot(opcode: &str, annots: Vec<&Annotation>) -> String {
    let field_annot = annots
        .into_iter()
        .filter(|a| a.kind() == Kind::Field)
        .last();
    match field_annot {
        Some(annot) => format!("{} %{}", opcode, annot.value()),
        None => opcode.to_string()
    }
}

impl Formatter for Instruction {
    fn format(&self) -> String {
        match self {
            Instruction::Push(_) => "Push".into(),
            Instruction::Drop(instr) => format_instr_n("Drop", instr.n.as_ref()),
            Instruction::Dup(instr) => format_instr_n("Dup", instr.n.as_ref()),
            Instruction::Swap(_) => "Swap".into(),
            Instruction::Dig(instr) => format_instr_n("Dig", Some(&instr.n)),
            Instruction::Dug(instr) => format_instr_n("Dug", Some(&instr.n)),
            Instruction::Rename(_) => "Rename".into(),
            Instruction::Cast(_) => "Cast".into(),
            Instruction::FailWith(_) => "FailWith".into(),
            Instruction::Dip(instr) => format_instr_n("Dip", instr.n.as_ref()),
            Instruction::If(_) => "If".into(),
            Instruction::IfCons(_) => "IfCons".into(),
            Instruction::IfLeft(_) => "IfLeft".into(),
            Instruction::IfNone(_) => "IfNone".into(),
            Instruction::Loop(_) => "Loop".into(),
            Instruction::LoopLeft(_) => "LoopLeft".into(),
            Instruction::Map(_) => "Map".into(),
            Instruction::Iter(_) => "Iter".into(),
            Instruction::Lambda(_) => "Lambda".into(),
            Instruction::Apply(_) => "Apply".into(),
            Instruction::Exec(_) => "Exec".into(),
            Instruction::Abs(_) => "Abs".into(),
            Instruction::Add(_) => "Add".into(),
            Instruction::Ediv(_) => "Ediv".into(),
            Instruction::Lsl(_) => "Lsl".into(),
            Instruction::Lsr(_) => "Lsr".into(),
            Instruction::Mul(_) => "Mul".into(),
            Instruction::Neg(_) => "Neg".into(),
            Instruction::Sub(_) => "Sub".into(),
            Instruction::SubMutez(_) => "SubMutez".into(),
            Instruction::Int(_) => "Int".into(),
            Instruction::IsNat(_) => "IsNat".into(),
            Instruction::Or(_) => "Or".into(),
            Instruction::Xor(_) => "Xor".into(),
            Instruction::And(_) => "And".into(),
            Instruction::Not(_) => "Not".into(),
            Instruction::Compare(_) => "Compare".into(),
            Instruction::Eq(_) => "Eq".into(),
            Instruction::Neq(_) => "Neq".into(),
            Instruction::Gt(_) => "Gt".into(),
            Instruction::Ge(_) => "Ge".into(),
            Instruction::Lt(_) => "Lt".into(),
            Instruction::Le(_) => "Le".into(),
            Instruction::Size(_) => "Size".into(),
            Instruction::Slice(_) => "Slice".into(),
            Instruction::Concat(_) => "Concat".into(),
            Instruction::Pack(_) => "Pack".into(),
            Instruction::Unpack(_) => "Unpack".into(),
            Instruction::Unit(_) => "Unit".into(),
            Instruction::Car(_) => "Car".into(),
            Instruction::Cdr(_) => "Cdr".into(),
            Instruction::Pair(instr) => format_instr_n("Pair", instr.n.as_ref()),
            Instruction::Unpair(instr) => format_instr_n("Unpair", instr.n.as_ref()),
            Instruction::None(_) => "None".into(),
            Instruction::Some(_) => "Some".into(),
            Instruction::Left(_) => "Left".into(),
            Instruction::Right(_) => "Right".into(),
            Instruction::Nil(_) => "Nil".into(),
            Instruction::Cons(_) => "Cons".into(),
            Instruction::EmptySet(_) => "EmptySet".into(),
            Instruction::EmptyMap(_) => "EmptyMap".into(),
            Instruction::EmptyBigMap(_) => "EmptyBigMap".into(),
            Instruction::Mem(_) => "Mem".into(),
            Instruction::Get(instr) => format_instr_n("Get", instr.n.as_ref()),
            Instruction::Update(instr) => format_instr_n("Update", instr.n.as_ref()),
            Instruction::GetAndUpdate(_) => "GetAndUpdate".into(),
            Instruction::Amount(_) => "Amount".into(),
            Instruction::Sender(_) => "Sender".into(),
            Instruction::Source(_) => "Source".into(),
            Instruction::Now(_) => "Now".into(),
            Instruction::Level(_) => "Level".into(),
            Instruction::SelfAddress(_) => "SelfAddress".into(),
            Instruction::Balance(_) => "Balance".into(),
            Instruction::Address(_) => "Address".into(),
            Instruction::Contract(instr) => format_instr_annot("Contract", instr.annotations()),
            Instruction::Self_(instr) => format_instr_annot("Self_", instr.annotations()),
            Instruction::ImplicitAccount(_) => "ImplicitAccount".into(),
            Instruction::TransferTokens(_) => "TransferTokens".into(),
            _ => format!("{:?}", self)
        }
    }
}

impl Formatter for ComparableType {
    fn format(&self) -> String {
        match self {
            ComparableType::Address(_) => "address".into(),
            ComparableType::Bool(_) => "bool".into(),
            ComparableType::Bytes(_) => "bytes".into(),
            ComparableType::ChainId(_) => "chain_id".into(),
            ComparableType::Int(_) => "int".into(),
            ComparableType::Key(_) => "key".into(),
            ComparableType::KeyHash(_) => "key_hash".into(),
            ComparableType::Mutez(_) => "mutez".into(),
            ComparableType::Nat(_) => "nat".into(),
            ComparableType::Signature(_) => "signature".into(),
            ComparableType::String(_) => "string".into(),
            ComparableType::Timestamp(_) => "timestamp".into(),
            ComparableType::Unit(_) => "unit".into(),
            ty => format!("{:?}", ty)
        }        
    }
}

impl Formatter for Type {
    fn format(&self) -> String {
        match self {
            Type::Comparable(ty) => ty.format(),
            Type::Option(ty) => format!("(option {})", ty.r#type.format()),
            Type::Or(ty) => format!("(or {} {})", ty.lhs.format(), ty.rhs.format()),
            Type::Pair(ty) => {
                let args: Vec<String> = ty.types.iter().map(|x| x.format()).collect();
                format!("(pair {})", args.join(" "))
            },
            Type::List(ty) => format!("(list {})", ty.r#type.format()),
            Type::Set(ty) => format!("(set {})", ty.r#type.format()),
            Type::Map(ty) => format!("(map {} {})", ty.key_type.format(), ty.value_type.format()),
            Type::BigMap(ty) => format!("(big_map {} {})", ty.key_type.format(), ty.value_type.format()),
            Type::Lambda(ty) => format!("(lambda {} {})", ty.parameter_type.format(), ty.return_type.format()),
            Type::Contract(ty) => format!("(contract {})", ty.r#type.format()),
            Type::Operation(_) => "operation".into(),
            Type::Parameter(ty) => format!("(parameter {})", ty.r#type.format()),
            Type::Storage(ty) => format!("(storage {})", ty.r#type.format()),
            ty => format!("{:?}", ty)
        }        
    }
}

impl Formatter for Data {
    fn format(&self) -> String {
        match self {
            Data::Unit(_) => "Unit".into(),
            Data::False(_) => "False".into(),
            Data::True(_) => "True".into(),
            Data::None(_) => "None".into(),
            Data::Bytes(val) => val.value().into(),
            Data::Int(val) => val.to_string(),
            Data::Nat(val) => val.to_string(),
            Data::String(val) => format!("\"{}\"", val.to_str()),
            Data::Some(val) => format!("(Some {})", val.value.format()),
            Data::Left(val) => format!("(Left {})", val.value.format()),
            Data::Right(val) => format!("(Right {})", val.value.format()),
            Data::Pair(val) => {
                let args: Vec<String> = val.values.iter().map(|x| x.format()).collect();
                format!("(Pair {})", args.join(" "))
            },
            Data::Sequence(val) => {
                let args: Vec<String> = val.values().iter().map(|x| x.format()).collect();
                format!("{{{}}}", args.join(" "))
            },
            Data::Elt(val) => format!("Elt {} {}", val.key.format(), val.value.format()),
            Data::Map(val) => {
                let args: Vec<String> = val.values().iter().map(|x| Data::Elt(x.clone()).format()).collect();
                format!("{{{}}}", args.join(" "))
            },
            Data::Instruction(val) => val.format(), // TODO: uppercase?
        }        
    }
}

