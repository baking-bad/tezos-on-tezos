use derive_more::{Display, Error};
use std::backtrace::Backtrace;
use tezos_l2::error::{TezosCoreError};

#[derive(Debug, Display)]
pub enum InternalKind {
    Context,
    TezosCore,
    SerdeJson,
    Reqwest,
    Ureq,
    StdNum,
    StdIO,
    Hex,
    Misc
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
    KeyNotFound,
    DurableStorageError {
        message: String
    },
    RollupClientError {
        status: u16
    }
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
                $crate::internal_error!($kind, "Caused by: {}", error.to_string())
            }
        }
    };
}

impl_from_error!(TezosCoreError, TezosCore);
impl_from_error!(std::num::ParseIntError, StdNum);
impl_from_error!(serde_json::Error, SerdeJson);
impl_from_error!(&str, Misc);
impl_from_error!(std::io::Error, StdIO);
impl_from_error!(hex::FromHexError, Hex);
impl_from_error!(reqwest::Error, Reqwest);

impl From<context::Error> for Error {
    fn from(error: context::Error) -> Self {
        internal_error!(Context, "Caused by: {}", error.format())
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
