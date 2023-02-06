use actix_web::{
    web::{Data, Json, Path},
    Responder, Result,
};
use serde::Deserialize;
use tezos_core::types::{
    encoded::{Address, BlockHash, ImplicitAddress, PublicKey},
    mutez::Mutez,
    number::Nat,
};
use tezos_michelson::micheline::{sequence::Sequence, Micheline};
use tezos_operation::operations::{
    Entrypoint, OperationContent, Origination, Parameters, Reveal, Script, SignedOperation,
    Transaction,
};

use crate::{json_response, rollup::TezosHelpers, Error};

pub const ZERO_SIGNATURE: &str =
    "sigMzJ4GVAvXEd2RjsKGfG2H9QvqTSKCZsuB2KiHbZRGFz72XgF6KaKADznh674fQgBatxw3xdHqTtMHUZAGRprxy64wg1aq";

#[derive(Deserialize, Debug)]
pub struct OriginationScript {
    pub code: Sequence,
    pub storage: Micheline,
}

#[derive(Deserialize, Debug)]
pub struct TransactionParameters {
    pub entrypoint: String,
    pub value: Micheline,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "kind")]
#[serde(rename_all = "snake_case")]
pub enum PartialContent {
    Reveal {
        source: ImplicitAddress,
        counter: Nat,
        public_key: PublicKey,
    },
    Transaction {
        source: ImplicitAddress,
        counter: Nat,
        amount: Mutez,
        destination: Address,
        parameters: Option<TransactionParameters>,
    },
    Origination {
        source: ImplicitAddress,
        counter: Nat,
        balance: Mutez,
        script: OriginationScript,
    },
}

#[derive(Deserialize, Debug)]
pub struct MaybeSignedOperation {
    pub branch: BlockHash,
    pub contents: Vec<PartialContent>,
}

#[derive(Deserialize, Debug)]
pub struct RunOperationRequest {
    operation: MaybeSignedOperation,
}

impl TryInto<OperationContent> for PartialContent {
    type Error = Error;

    fn try_into(self) -> crate::Result<OperationContent> {
        match self {
            Self::Reveal {
                source,
                counter,
                public_key,
            } => Ok(OperationContent::Reveal(Reveal {
                source,
                counter,
                public_key,
                fee: 0u32.into(),
                gas_limit: 0u32.into(),
                storage_limit: 0u32.into(),
            })),
            Self::Transaction {
                source,
                counter,
                amount,
                destination,
                parameters,
            } => Ok(OperationContent::Transaction(Transaction {
                source,
                counter,
                amount,
                destination,
                parameters: parameters.map(|p| Parameters {
                    entrypoint: Entrypoint::from_str(&p.entrypoint),
                    value: p.value,
                }),
                fee: 0u32.into(),
                gas_limit: 0u32.into(),
                storage_limit: 0u32.into(),
            })),
            Self::Origination {
                source,
                counter,
                balance,
                script,
            } => Ok(OperationContent::Origination(Origination {
                source,
                counter,
                balance,
                script: Script {
                    code: script.code,
                    storage: script.storage,
                },
                delegate: None,
                fee: 0u32.into(),
                gas_limit: 0u32.into(),
                storage_limit: 0u32.into(),
            })),
        }
    }
}

impl TryInto<SignedOperation> for RunOperationRequest {
    type Error = Error;

    fn try_into(self) -> crate::Result<SignedOperation> {
        let contents: crate::Result<Vec<OperationContent>> = self
            .operation
            .contents
            .into_iter()
            .map(|content| content.try_into())
            .collect();
        Ok(SignedOperation {
            branch: self.operation.branch,
            contents: contents?,
            signature: ZERO_SIGNATURE.try_into().unwrap(),
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
    Ok(json_response!(value))
}
