use actix_web::{
    http::StatusCode,
    web::{Data, Json, Path},
    HttpResponse, Responder, Result,
};
use serde::Deserialize;
use tezos_core::types::encoded::ChainId;
use tezos_operation::operations::{OperationContent, SignedOperation};
use tezos_rpc::models::operation::Operation;

use crate::{rollup::TezosHelpers, Error};

pub const ZERO_SIGNATURE: &str =
    "sigMzJ4GVAvXEd2RjsKGfG2H9QvqTSKCZsuB2KiHbZRGFz72XgF6KaKADznh674fQgBatxw3xdHqTtMHUZAGRprxy64wg1aq";

#[derive(Deserialize)]
pub struct RunOperationRequest {
    operation: Operation,
    chain_id: Option<ChainId>,
}

impl TryInto<SignedOperation> for RunOperationRequest {
    type Error = Error;

    fn try_into(self) -> crate::Result<SignedOperation> {
        let contents: tezos_rpc::Result<Vec<OperationContent>> = self
            .operation
            .contents
            .into_iter()
            .map(|content| OperationContent::try_from(content))
            .collect();
        Ok(SignedOperation {
            branch: self.operation.branch,
            signature: self
                .operation
                .signature
                .unwrap_or(ZERO_SIGNATURE.try_into().unwrap()),
            contents: contents?,
        })
    }
}

pub async fn run_operation<T: TezosHelpers>(
    client: Data<T>,
    path: Path<(String,)>,
    request: Json<RunOperationRequest>,
) -> Result<impl Responder> {
    let value = client
        .simulate_operation(&path.0.as_str().try_into()?, request.0.try_into()?)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}
