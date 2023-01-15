pub mod typechecker;
pub mod interpreter;
pub mod formatter;
pub mod stack;
pub mod types;
pub mod instructions;
pub mod script;
pub mod tracer;
pub mod error;

pub use error::{Error, Result};

#[cfg(feature = "trace")]
pub use tracer::{trace_init, trace_into, trace_stack, trace_err, trace_ret, trace_log};

#[macro_export]
macro_rules! trace_log {
    ($cmd: expr, $arg: expr) => {
        #[cfg(feature = "trace")]
        $crate::trace_log(format!("{} {}", $cmd, $arg));
    };
    ($err: expr) => {
        #[cfg(feature = "trace")]
        $crate::trace_err($err);
    };
}

#[macro_export]
macro_rules! trace_stack {
    ($cmd: expr, $item: expr, $arg: expr) => {
        #[cfg(feature = "trace")]
        $crate::trace_stack($cmd, $item, $arg);
    };
    ($cmd: expr, $item: expr) => {
        #[cfg(feature = "trace")]
        $crate::trace_stack($cmd, $item, None);
    };
}

#[macro_export]
macro_rules! trace_enter {
    () => {
        #[cfg(feature = "trace")]
        $crate::trace_init();  
    };
    ($msg: literal) => {
        #[cfg(feature = "trace")]
        $crate::trace_into(None, Some($msg));
    };
    ($instr: ident) => {
        #[cfg(feature = "trace")]
        $crate::trace_into(Some($instr), None);
    };
}

#[macro_export]
macro_rules! trace_exit {
    () => {
        #[cfg(feature = "trace")]
        $crate::trace_ret(None, None);
    };
    ($msg: literal) => {
        #[cfg(feature = "trace")]
        $crate::trace_ret(None, Some($msg));
    };
    ($maybe_err: expr) => {
        #[cfg(feature = "trace")]
        $crate::trace_ret($maybe_err, None);
    };
    ($maybe_err: expr, $msg: expr) => {
        #[cfg(feature = "trace")]
        $crate::trace_ret($maybe_err, Some($msg));
    };
}