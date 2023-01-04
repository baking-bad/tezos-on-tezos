use once_cell::sync::Lazy;
use tezos_michelson::michelson::{
    data::Instruction,
    annotations::{Kind, Annotation}
};
use tezos_core::types::number::Nat;

use crate::{Error, types::StackItem};

const OUTER: &str = "│ ";
const INNER: &str = "├ ";
const RET: &str  = "└ ";

fn format_n(opcode: &str, arg: Option<&Nat>) -> String {
    match arg {
        Some(arg) => format!("{} {}", opcode, arg.to_str()),
        None => opcode.to_string(),
    }
}

fn format_annot(opcode: &str, annots: Vec<&Annotation>) -> String {
    let field_annot = annots
        .into_iter()
        .filter(|a| a.kind() == Kind::Field)
        .last();
    match field_annot {
        Some(annot) => format!("{} %{}", opcode, annot.value()),
        None => opcode.to_string()
    }
}

fn format_instr(instr: &Instruction) -> String {
    match instr {
        Instruction::Push(_) => "Push".into(),
        Instruction::Drop(instr) => format_n("Drop", instr.n.as_ref()),
        Instruction::Dup(instr) => format_n("Dup", instr.n.as_ref()),
        Instruction::Swap(_) => "Swap".into(),
        Instruction::Dig(instr) => format_n("Dig", Some(&instr.n)),
        Instruction::Dug(instr) => format_n("Dug", Some(&instr.n)),
        Instruction::Rename(_) => "Rename".into(),
        Instruction::Cast(_) => "Cast".into(),
        Instruction::FailWith(_) => "FailWith".into(),
        Instruction::Dip(instr) => format_n("Dip", instr.n.as_ref()),
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
        Instruction::Pair(instr) => format_n("Pair", instr.n.as_ref()),
        Instruction::Unpair(instr) => format_n("Unpair", instr.n.as_ref()),
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
        Instruction::Get(instr) => format_n("Get", instr.n.as_ref()),
        Instruction::Update(instr) => format_n("Update", instr.n.as_ref()),
        Instruction::GetAndUpdate(_) => "GetAndUpdate".into(),
        Instruction::Amount(_) => "Amount".into(),
        Instruction::Sender(_) => "Sender".into(),
        Instruction::Source(_) => "Source".into(),
        Instruction::Now(_) => "Now".into(),
        Instruction::Level(_) => "Level".into(),
        Instruction::SelfAddress(_) => "SelfAddress".into(),
        Instruction::Balance(_) => "Balance".into(),
        Instruction::Address(_) => "Address".into(),
        Instruction::Contract(instr) => format_annot("Contract", instr.annotations()),
        Instruction::Self_(instr) => format_annot("Self_", instr.annotations()),
        Instruction::ImplicitAccount(_) => "ImplicitAccount".into(),
        _ => format!("{:?}", instr)
    }
}

pub struct Tracer {
    depth: usize
}

impl Tracer {
    pub fn new() -> Self {
        Self { depth: 0 }
    }

    fn write(&self, delim: &str, msg: &str) {
        for i in 0..self.depth {
            print!("\x1b[{}m{}", 91 + i, OUTER);
        }
        println!("\x1b[{}m{}\x1b[0m{}", 91 + self.depth, delim, msg);
    }

    pub fn init(&mut self) {
        println!("\n\x1b[1m\x1b[91mTrace");
        self.depth = 0;
    }

    pub fn log(&self, msg: String) {
        println!("{}{}{}", OUTER.repeat(self.depth), INNER, msg)
    }

    pub fn step_into(&mut self, instr: Option<&Instruction>, msg: Option<&str>) {
        match instr {
            None => {
                if let Some(msg) = msg {
                    self.write(INNER, format!("\x1b[{}m{}", 92 + self.depth, msg).as_str());
                    self.depth += 1;
                }
            },
            Some(Instruction::Sequence(_)) => {},
            Some(instr) => {
                self.write(INNER, format!("\x1b[1m\x1b[{}m{}", 92 + self.depth, format_instr(instr)).as_str());
                self.depth += 1;
            }
        }
    }

    pub fn step_out(&mut self, err: Option<&Error>, msg: Option<&str>) {
        match err {
            Some(err) => self.write(RET, format!("\x1b[{}mErr {:?}\x1b[0m", 91 + self.depth, err).as_str()),
            None => self.write(RET, format!("\x1b[{}m{}\x1b[0m", 91 + self.depth, msg.unwrap_or("Ok")).as_str())
        }
        if self.depth > 0 {
            self.depth -= 1;
        }
    }

    pub fn step_over(&self, cmd: &str, item: &StackItem, arg: Option<&usize>) {
        match arg {
            Some(arg) => self.write(INNER, format!("\x1b[1m{}@{}\x1b[0m {}", cmd, arg, item).as_str()),
            None => self.write(INNER, format!("\x1b[1m{}\x1b[0m {}", cmd, item).as_str())
        }
    }
}

pub static mut TRACER: Lazy<Tracer> = Lazy::new(|| Tracer::new());

pub fn trace_init() {
    unsafe {
        TRACER.init();
    }
}

pub fn trace_into(instr: Option<&Instruction>, msg: Option<&str>) {
    unsafe {
        TRACER.step_into(instr, msg);
    }
}

pub fn trace_stack(cmd: &str, item: &StackItem, arg: Option<&usize>) {
    unsafe {
        TRACER.step_over(cmd, item, arg);
    }
}

pub fn trace_err(err: &Error) {
    unsafe {
        TRACER.log(format!("Warn {}", err))
    }
}

pub fn trace_ret(err: Option<&Error>, msg: Option<&str>) {
    unsafe {
        TRACER.step_out(err, msg);
    }
}