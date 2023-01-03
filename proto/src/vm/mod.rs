mod typechecker;
mod interpreter;
mod stack;
mod types;
mod instructions;
mod script;
mod tracer;

pub use types::StackItem;
pub use stack::Stack;
pub use interpreter::{Interpreter, TransactionScope, TransactionResult};

#[cfg(feature = "trace")]
pub use tracer::{trace_init, trace_into, trace_stack, trace_err, trace_ret};

#[macro_export]
macro_rules! trace_log {
    ($cmd: expr, $item: expr, $arg: expr) => {
        #[cfg(feature = "trace")]
        $crate::vm::trace_stack($cmd, $item, $arg);
    };
    ($cmd: expr, $item: expr) => {
        #[cfg(feature = "trace")]
        $crate::vm::trace_stack($cmd, $item, None);
    };
    ($err: expr) => {
        #[cfg(feature = "trace")]
        $crate::vm::trace_err($err);
    };
}

#[macro_export]
macro_rules! trace_enter {
    () => {
        #[cfg(feature = "trace")]
        $crate::vm::trace_init();  
    };
    ($msg: literal) => {
        #[cfg(feature = "trace")]
        $crate::vm::trace_into(None, Some($msg));
    };
    ($instr: ident) => {
        #[cfg(feature = "trace")]
        $crate::vm::trace_into(Some($instr), None);
    };
}

#[macro_export]
macro_rules! trace_exit {
    () => {
        #[cfg(feature = "trace")]
        $crate::vm::trace_ret(None, None);
    };
    ($msg: literal) => {
        #[cfg(feature = "trace")]
        $crate::vm::trace_ret(None, Some($msg));
    };
    ($maybe_err: expr) => {
        #[cfg(feature = "trace")]
        $crate::vm::trace_ret($maybe_err, None);
    };
    ($maybe_err: expr, $msg: expr) => {
        #[cfg(feature = "trace")]
        $crate::vm::trace_ret($maybe_err, Some($msg));
    };
}