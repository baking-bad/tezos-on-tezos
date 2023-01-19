use derive_more::{Display, Error};
use proto::error::{Error as TezosProtoError, TezosCoreError, TezosOperationError};
use std::backtrace::Backtrace;

#[derive(Debug, Display)]
pub enum InternalKind {
    WasmHost,
    TezosEncoding,
    TezosProtocol,
}

#[derive(Debug)]
pub struct InternalError {
    pub kind: InternalKind,
    pub message: String,
    pub backtrace: Backtrace,
}

impl InternalError {
    pub fn new(kind: InternalKind, message: String) -> Self {
        Self {
            kind,
            message,
            backtrace: Backtrace::capture(),
        }
    }

    pub fn format(&self) -> String {
        format!(
            "{} error\n{}\nStacktrace:\n{}",
            self.kind, self.message, self.backtrace
        )
    }
}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} error", self.kind))
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
    #[display(fmt = "UnexpectedL2OperationLength")]
    UnexpectedL2OperationLength {
        length: usize,
    },
    #[display(fmt = "UnexpectedLevelInfoLength")]
    UnexpectedLevelInfoLength {
        length: usize,
    },
    #[display(fmt = "InconsistentHeadLevel")]
    InconsistentHeadLevel {
        expected: i32,
        found: i32,
    },
    #[display(fmt = "InconsistentHeadTimestamp")]
    InconsistentHeadTimestamp {
        upper_bound: i64,
        found: i64,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! internal_error {
    ($kind: ident, $($arg:tt)*) => {
        $crate::Error::Internal(
            $crate::error::InternalError::new(
                $crate::error::InternalKind::$kind,
                format!($($arg)*)
            )
        )
    };
}

macro_rules! impl_from_error {
    ($inner_err_ty: ty, $kind: ident) => {
        impl From<$inner_err_ty> for Error {
            fn from(error: $inner_err_ty) -> Self {
                $crate::internal_error!($kind, "Caused by: {:?}", error)
            }
        }
    };
}

impl_from_error!(TezosCoreError, TezosEncoding);
impl_from_error!(TezosOperationError, TezosEncoding);
impl_from_error!(host::runtime::RuntimeError, WasmHost);
impl_from_error!(host::path::PathError, WasmHost);

impl From<TezosProtoError> for Error {
    fn from(error: TezosProtoError) -> Self {
        internal_error!(TezosProtocol, "Caused by: {}", error.format())
    }
}

impl Error {
    pub fn format(&self) -> String {
        match self {
            Self::Internal(internal) => internal.format(),
            err => format!("{:#?}", err),
        }
    }
}
