use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use std::backtrace::Backtrace;
use tezos_rpc::models::error::RpcError;

#[derive(Debug, Display)]
pub enum InternalKind {
    Context,
    Interpreter,
    TezosRpc,
    TezosCore,
    TezosOperation,
    TezosMichelson,
    TezosProtocol,
    SerdeJson,
    Reqwest,
    StdNum,
    StdIO,
    Hex,
    IBig,
    Actix,
    Misc,
    Sync,
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
    KeyNotFound { key: String },
    RollupInternalError { message: String },
    RollupClientError { status: u16 },
    InvalidArguments { message: String },
    RpcErrors { response: String },
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

impl_from_error!(tezos_rpc::Error, TezosRpc);
impl_from_error!(tezos_core::Error, TezosCore);
impl_from_error!(tezos_operation::Error, TezosOperation);
impl_from_error!(tezos_michelson::Error, TezosMichelson);
impl_from_error!(tezos_l2::Error, TezosProtocol);
impl_from_error!(std::num::ParseIntError, StdNum);
impl_from_error!(std::num::TryFromIntError, StdNum);
impl_from_error!(serde_json::Error, SerdeJson);
impl_from_error!(&str, Misc);
impl_from_error!(std::io::Error, StdIO);
impl_from_error!(hex::FromHexError, Hex);
impl_from_error!(reqwest::Error, Reqwest);
impl_from_error!(ibig::error::ParseError, IBig);
impl_from_error!(actix_web::rt::task::JoinError, Actix);

impl From<tezos_ctx::Error> for Error {
    fn from(error: tezos_ctx::Error) -> Self {
        internal_error!(Context, "Caused by: {}", error.format())
    }
}

impl From<michelson_vm::Error> for Error {
    fn from(error: michelson_vm::Error) -> Self {
        internal_error!(Context, "Caused by: {}", error.format())
    }
}

impl From<Vec<RpcError>> for Error {
    fn from(errors: Vec<RpcError>) -> Self {
        Error::RpcErrors {
            response: serde_json::to_string(&errors).expect("Failed to serialize RPC errors"),
        }
    }
}

impl Error {
    pub fn format(&self) -> String {
        match self {
            Self::Internal(internal) => internal.format(),
            Self::RpcErrors { response } => response.clone(),
            err => format!("{:#?}", err),
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::KeyNotFound { key: _ } => StatusCode::NOT_FOUND,
            Error::InvalidArguments { message: _ } => StatusCode::BAD_REQUEST,
            Error::Internal(int) => match int.kind {
                InternalKind::TezosCore => StatusCode::BAD_REQUEST,
                InternalKind::TezosOperation => StatusCode::BAD_REQUEST,
                InternalKind::Hex => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.format())
    }
}
