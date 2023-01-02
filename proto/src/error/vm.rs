use derive_more::{Display, Error};
use tezos_michelson::michelson::{
    types::Type,
    data::Instruction
};
use tezos_michelson::micheline::Micheline;

#[derive(Debug, Display, Error, PartialEq)]
pub enum InterpreterError {
    #[display(fmt = "expected {}, found {}", expected, found)]
    TypeMismatch {
        expected: String,
        found: String
    },
    #[display(fmt = "{:?}", ty)]
    MichelsonTypeUnsupported {
        ty: Type
    },
    #[display(fmt = "{:?}", instruction)]
    MichelsonInstructionUnsupported {
        instruction: Instruction
    },
    #[display(fmt = "{:?}", with)]
    ScriptFailed {
        with: Micheline
    },
    MissingScriptField {
        prim: String
    },
    BadStack {
        location: usize
    },
    BadReturn,
    #[display(fmt = "expected {}, found: {}", expected, found)]
    InvalidArity {
        expected: usize,
        found: usize
    },
    GeneralOverflow,
    MutezOverflow,
    MutezUnderflow,
}

#[macro_export]
macro_rules! err_type {
    ($expected: expr, $found: expr) => {
        Err(crate::error::Error::InterpreterError(
            crate::error::InterpreterError::TypeMismatch {
                expected: format!("{:?}", $expected),
                found: format!("{:?}", $found)
            }
        ))
    };
}
