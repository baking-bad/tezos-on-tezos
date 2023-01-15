use once_cell::sync::Lazy;
use tezos_michelson::michelson::{
    data::Instruction,
};

use crate::{
    Error,
    types::StackItem,
    formatter::Formatter
};

const OUTER: &str = "│ ";
const INNER: &str = "├ ";
const RET: &str   = "└ ";

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
                self.write(INNER, format!("\x1b[1m\x1b[{}m{}", 92 + self.depth, instr.format()).as_str());
                self.depth += 1;
            }
        }
    }

    pub fn step_out(&mut self, err: Option<&Error>, msg: Option<&str>) {
        match err {
            Some(err) => self.write(RET, format!("\x1b[{}mErr {}\x1b[0m", 91 + self.depth, err).as_str()),
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

pub fn trace_log(msg: String) {
    unsafe {
        TRACER.log(msg)
    }
}