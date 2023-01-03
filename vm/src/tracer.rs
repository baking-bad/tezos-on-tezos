use once_cell::sync::Lazy;
use tezos_michelson::michelson::data::Instruction;
use tezos_core::types::number::Nat;

use crate::{Error, types::StackItem};

const OUTER: &str = "│ ";
const INNER: &str = "├ ";
const RET: &str  = "└ ";

fn format_instr(opcode: &str, arg: Option<&Nat>) -> String {
    match arg {
        Some(arg) => format!("{} {}", opcode, arg.to_str()),
        None => opcode.to_string(),
    }
}

fn display_instr(instr: &Instruction) -> String {
    match instr {
        Instruction::Push(_) => format_instr("Push", None),
        Instruction::Drop(instr) => format_instr("Drop", instr.n.as_ref()),
        Instruction::Dup(instr) => format_instr("Dup", instr.n.as_ref()),
        Instruction::Swap(_) => format_instr("Swap", None),
        Instruction::Dig(instr) => format_instr("Dig", Some(&instr.n)),
        Instruction::Dug(instr) => format_instr("Dug", Some(&instr.n)),
        Instruction::Rename(_) => format_instr("Rename", None),
        Instruction::Cast(_) => format_instr("Cast", None),
        Instruction::FailWith(_) => format_instr("FailWith", None),
        Instruction::Dip(instr) => format_instr("Dip", instr.n.as_ref()),
        Instruction::If(_) => format_instr("If", None),
        Instruction::IfCons(_) => format_instr("IfCons", None),
        Instruction::IfLeft(_) => format_instr("IfLeft", None),
        Instruction::IfNone(_) => format_instr("IfNone", None),
        Instruction::Loop(_) => format_instr("Loop", None),
        Instruction::LoopLeft(_) => format_instr("LoopLeft", None),
        Instruction::Map(_) => format_instr("Map", None),
        Instruction::Iter(_) => format_instr("Iter", None),
        Instruction::Lambda(_) => format_instr("Lambda", None),
        Instruction::Apply(_) => format_instr("Apply", None),
        Instruction::Exec(_) => format_instr("Exec", None),
        Instruction::Abs(_) => format_instr("Abs", None),
        Instruction::Add(_) => format_instr("Add", None),
        Instruction::Ediv(_) => format_instr("Ediv", None),
        Instruction::Lsl(_) => format_instr("Lsl", None),
        Instruction::Lsr(_) => format_instr("Lsr", None),
        Instruction::Mul(_) => format_instr("Mul", None),
        Instruction::Neg(_) => format_instr("Neg", None),
        Instruction::Sub(_) => format_instr("Sub", None),
        Instruction::Int(_) => format_instr("Int", None),
        Instruction::IsNat(_) => format_instr("IsNat", None),
        Instruction::Or(_) => format_instr("Or", None),
        Instruction::Xor(_) => format_instr("Xor", None),
        Instruction::And(_) => format_instr("And", None),
        Instruction::Not(_) => format_instr("Not", None),
        Instruction::Compare(_) => format_instr("Compare", None),
        Instruction::Eq(_) => format_instr("Eq", None),
        Instruction::Neq(_) => format_instr("Neq", None),
        Instruction::Gt(_) => format_instr("Gt", None),
        Instruction::Ge(_) => format_instr("Ge", None),
        Instruction::Lt(_) => format_instr("Lt", None),
        Instruction::Le(_) => format_instr("Le", None),
        Instruction::Size(_) => format_instr("Size", None),
        Instruction::Slice(_) => format_instr("Slice", None),
        Instruction::Concat(_) => format_instr("Concat", None),
        Instruction::Pack(_) => format_instr("Pack", None),
        Instruction::Unpack(_) => format_instr("Unpack", None),
        Instruction::Unit(_) => format_instr("Unit", None),
        Instruction::Car(_) => format_instr("Car", None),
        Instruction::Cdr(_) => format_instr("Cdr", None),
        Instruction::Pair(instr) => format_instr("Pair", instr.n.as_ref()),
        Instruction::Unpair(instr) => format_instr("Unpair", instr.n.as_ref()),
        Instruction::None(_) => format_instr("None", None),
        Instruction::Some(_) => format_instr("Some", None),
        Instruction::Left(_) => format_instr("Left", None),
        Instruction::Right(_) => format_instr("Right", None),
        Instruction::Nil(_) => format_instr("Nil", None),
        Instruction::Cons(_) => format_instr("Cons", None),
        Instruction::EmptySet(_) => format_instr("EmptySet", None),
        Instruction::EmptyMap(_) => format_instr("EmptyMap", None),
        Instruction::Mem(_) => format_instr("Mem", None),
        Instruction::Get(instr) => format_instr("Get", instr.n.as_ref()),
        Instruction::Update(instr) => format_instr("Update", instr.n.as_ref()),
        Instruction::GetAndUpdate(_) => format_instr("GetAndUpdate", None),
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
        //let (color_open, color_close) = COLORS[self.depth % 1];
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
                self.write(INNER, format!("\x1b[1m\x1b[{}m{}", 92 + self.depth, display_instr(instr)).as_str());
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