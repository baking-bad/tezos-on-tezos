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

    pub fn format(&self) -> String {
        format!(
            "Layered storage error\n{}\nStacktrace:\n{}",
            self.message, self.backtrace
        )
    }
}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Layered storage error, {}",
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
    ContextUnstagedError,
    DowncastingError,
}

pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! internal_error {
    ($($arg:tt)*) => {
        $crate::Error::Internal(
            $crate::error::InternalError::new( format!($($arg)*))
        )
    };
}

macro_rules! impl_from_error {
    ($inner_err_ty: ty) => {
        impl From<$inner_err_ty> for Error {
            fn from(error: $inner_err_ty) -> Self {
                $crate::internal_error!("{:?}", error)
            }
        }
    };
}

impl_from_error!(&str);

#[cfg(any(test, feature = "kernel"))]
impl_from_error!(tezos_smart_rollup_host::runtime::RuntimeError);

impl Error {
    pub fn format(&self) -> String {
        match self {
            Self::Internal(internal) => internal.format(),
            err => format!("{:#?}", err),
        }
    }
}

pub fn err_into(e: impl std::fmt::Debug) -> Error {
    Error::Internal(InternalError::new(format!("{:?}", e)))
}
