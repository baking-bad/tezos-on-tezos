use derive_more::{Display, Error};
use std::backtrace::Backtrace;

#[derive(Debug)]
pub struct InternalError {
    pub message: String,
    pub backtrace: Backtrace,
}

impl InternalError {
    pub fn new(message: String) -> Self {
        Self {
            message,
            backtrace: Backtrace::capture(),
        }
    }

    pub fn wrap(error: impl std::error::Error) -> Self {
        Self::new(error.to_string())
    }

    pub fn format(&self) -> String {
        format!(
            "Michelson Interop error\n{}\nStacktrace:\n{}",
            self.message, self.backtrace
        )
    }
}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Michelson Interop error, {}",
            self.message.replace("\n", " ")
        ))
    }
}

impl std::error::Error for InternalError {
    fn description(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Display, Error)]
pub enum Error {
    Internal(InternalError),
    TypeMismatch {
        message: String
    },
    CastError {
        message: String
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! internal_error {
    ($($arg:tt)*) => {
        $crate::Error::Internal(
            $crate::error::InternalError::new(format!($($arg)*))
        )
    };
}

macro_rules! impl_from_error {
    ($inner_err_ty: ty) => {
        impl From<$inner_err_ty> for Error {
            fn from(error: $inner_err_ty) -> Self {
                $crate::error::Error::Internal(InternalError::wrap(error))
            }
        }
    };
}

impl_from_error!(tezos_core::Error);
impl_from_error!(tezos_michelson::Error);