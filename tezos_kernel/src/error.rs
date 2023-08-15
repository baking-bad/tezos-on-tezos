// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

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
            "Kernel internal error\n{}\nStacktrace:\n{}",
            self.message, self.backtrace
        )
    }
}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Kernel internal error, {}",
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
    #[display(fmt = "UnexpectedL2OperationLength")]
    UnexpectedL2OperationLength {
        length: usize,
    },
    #[display(fmt = "InconsistentHeadLevel")]
    InconsistentHeadLevel {
        expected: i32,
        found: i32,
    },
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
                $crate::internal_error!("Caused by: {:?}", error)
            }
        }
    };
}

impl_from_error!(tezos_operation::Error);
impl_from_error!(tezos_core::Error);
impl_from_error!(tezos_smart_rollup_host::runtime::RuntimeError);
impl_from_error!(tezos_smart_rollup_host::path::PathError);
impl_from_error!(kernel_io::Error);
impl_from_error!(layered_store::Error);


impl From<tezos_proto::Error> for Error {
    fn from(error: tezos_proto::Error) -> Self {
        internal_error!("Caused by: {}", error)
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
