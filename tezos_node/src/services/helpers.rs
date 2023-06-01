use actix_web::{
    error::ErrorInternalServerError,
    web::{Data, Json, Path},
    Responder, Result,
};
use serde::{Deserialize, Deserializer};
use tezos_core::types::{
    encoded::{Address, BlockHash, ImplicitAddress, PublicKey},
    mutez::Mutez,
    number::Nat,
};
use tezos_michelson::micheline::{sequence::Sequence, Micheline};
use tezos_operation::operations::{
    Entrypoint, Operation, OperationContent, Origination, Parameters, Reveal, Script,
    SignedOperation, Transaction,
};

use crate::{json_response, rollup::TezosHelpers, Error};

pub const ZERO_SIGNATURE: &str =
    "sigMzJ4GVAvXEd2RjsKGfG2H9QvqTSKCZsuB2KiHbZRGFz72XgF6KaKADznh674fQgBatxw3xdHqTtMHUZAGRprxy64wg1aq";

#[derive(Deserialize, Clone)]
pub struct ParametersRequest {
    pub entrypoint: String,
    pub value: Micheline,
}

pub fn deserialize_params_opt<'de, D>(deserializer: D) -> Result<Option<Parameters>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<ParametersRequest>::deserialize(deserializer).map(|req: Option<ParametersRequest>| {
        match req {
            Some(params) => Some(Parameters {
                entrypoint: Entrypoint::from_str(&params.entrypoint),
                value: params.value,
            }),
            None => None,
        }
    })
}

#[derive(Deserialize, Clone)]
#[serde(remote = "Script")]
pub struct ScriptRequest {
    pub code: Sequence,
    pub storage: Micheline,
}

#[derive(Deserialize, Clone)]
#[serde(remote = "Reveal")]
struct RevealRequest {
    pub source: ImplicitAddress,
    pub fee: Mutez,
    pub counter: Nat,
    pub gas_limit: Nat,
    pub storage_limit: Nat,
    pub public_key: PublicKey,
}

#[derive(Deserialize, Clone)]
#[serde(remote = "Transaction")]
pub struct TransactionRequest {
    pub source: ImplicitAddress,
    pub fee: Mutez,
    pub counter: Nat,
    pub gas_limit: Nat,
    pub storage_limit: Nat,
    pub amount: Mutez,
    pub destination: Address,
    #[serde(default, deserialize_with = "deserialize_params_opt")]
    pub parameters: Option<Parameters>,
}

#[derive(Deserialize, Clone)]
#[serde(remote = "Origination")]
pub struct OriginationRequest {
    pub source: ImplicitAddress,
    pub fee: Mutez,
    pub counter: Nat,
    pub gas_limit: Nat,
    pub storage_limit: Nat,
    pub balance: Mutez,
    pub delegate: Option<ImplicitAddress>,
    #[serde(with = "ScriptRequest")]
    pub script: Script,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "kind")]
#[serde(rename_all = "snake_case")]
pub enum RequestContent {
    #[serde(with = "RevealRequest")]
    Reveal(Reveal),
    #[serde(with = "TransactionRequest")]
    Transaction(Transaction),
    #[serde(with = "OriginationRequest")]
    Origination(Origination),
}
#[derive(Deserialize, Debug, Clone)]
pub struct OperationRequest {
    pub branch: BlockHash,
    pub contents: Vec<RequestContent>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RunOperationRequest {
    operation: OperationRequest,
}

impl TryInto<OperationContent> for RequestContent {
    type Error = Error;

    fn try_into(self) -> crate::Result<OperationContent> {
        match self {
            Self::Reveal(content) => Ok(content.into()),
            Self::Transaction(content) => Ok(content.into()),
            Self::Origination(content) => Ok(content.into()),
        }
    }
}

impl TryInto<SignedOperation> for OperationRequest {
    type Error = Error;

    fn try_into(self) -> crate::Result<SignedOperation> {
        let contents: crate::Result<Vec<OperationContent>> = self
            .contents
            .into_iter()
            .map(|content| content.try_into())
            .collect();
        Ok(SignedOperation {
            branch: self.branch,
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
        .simulate_operation(
            &path.0.as_str().try_into()?,
            request.0.operation.try_into()?,
        )
        .await?;
    Ok(json_response!(value))
}

pub async fn preapply_operations<T: TezosHelpers>(
    client: Data<T>,
    path: Path<(String,)>,
    request: Json<Vec<OperationRequest>>,
) -> Result<impl Responder> {
    // validate that there is only one operation in array
    let operation: SignedOperation = request.0.clone().remove(0).try_into()?;
    let value = client
        .simulate_operation(&path.0.as_str().try_into()?, operation)
        .await?;
    Ok(json_response!(vec![value]))
}

pub async fn forge_operation<T: TezosHelpers>(
    request: Json<OperationRequest>,
) -> Result<impl Responder> {
    let operation: SignedOperation = request.0.try_into()?;
    let value = hex::encode(
        operation
            .to_forged_bytes()
            .map_err(ErrorInternalServerError)?,
    );
    Ok(json_response!(value))
}

#[cfg(test)]
mod test {
    use actix_web::{test, web::Data, App, http::header::ContentType};
    use serde_json::json;
    use tezos_core::types::{mutez::Mutez, encoded::{Encoded, PublicKey}};
    use tezos_ctx::{ExecutorContext, Head, GenericContext};
    use tezos_rpc::models::{error::RpcError, operation::Operation};
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;

    use crate::{rollup::mock_client::RollupMockClient, services::config, Result};

    #[actix_web::test]
    async fn test_forge_operation() -> Result<()> {
        let client = RollupMockClient::default();
        client.patch(|context| {
            context.set_head(Head::default()).unwrap();
            Ok(())
        })?;

        let app = test::init_service(
            App::new()
                .configure(config::<RollupMockClient>)
                .app_data(Data::new(client)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/chains/main/blocks/head/helpers/forge/operations")
            .set_json(json!({
                "branch": "BKoTRgyCZxBavMzKgTzaxVk3nKayFpFMr7QRZB16hzji7GGyEg6",
                "contents": [{
                    "kind": "transaction",
                    "source": "tz1dShXbbgJ4i1L6KMWAuuNdBPk5xCM1NRrU",
                    "fee": "471",
                    "counter": "80342938",
                    "gas_limit": "1546",
                    "storage_limit": "0",
                    "amount": "0",
                    "destination": "KT1GszRPFC31pjKXuRfTU53BfFhx3vwqK3bZ",
                    "parameters": {
                        "entrypoint": "provide_entropy",
                        "value": {
                            "int": "2338650231"
                        }
                    },
                }]
            }))
            .to_request();
        let res: String = test::call_and_read_body_json(&app, req).await;
        assert_eq!(
            "0bbf872a8a1e553381bde7f7e05df77ee41e69ca2df91a1cac26dd5af88ab82d6c00c34db0eaef2592adf9d0da9c94eab6a6bfc7070ad7039adfa7268a0c0000015b02cf10d7b3a6550e673bd04c76ef4106d7266e00ffff0f70726f766964655f656e74726f70790000000600b7e1a7b611",
            res.as_str()
        );
        Ok(())
    }

    #[actix_web::test]
    async fn test_preapply_operation() -> Result<()> {
        let client = RollupMockClient::default();
        client.patch(|context| {
            context.set_head(Head::default()).unwrap();
            Ok(())
        })?;

        let app = test::init_service(
            App::new()
                .configure(config::<RollupMockClient>)
                .app_data(Data::new(client)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/chains/main/blocks/head/helpers/preapply/operations")
            .set_json(json!([{
                "branch": "BKoTRgyCZxBavMzKgTzaxVk3nKayFpFMr7QRZB16hzji7GGyEg6",
                "contents": [{
                    "kind": "transaction",
                    "source": "tz1dShXbbgJ4i1L6KMWAuuNdBPk5xCM1NRrU",
                    "fee": "471",
                    "counter": "80342938",
                    "gas_limit": "1546",
                    "storage_limit": "0",
                    "amount": "0",
                    "destination": "KT1GszRPFC31pjKXuRfTU53BfFhx3vwqK3bZ",
                }]
            }]))
            .to_request();
        let res: Vec<RpcError> = test::call_and_read_body_json(&app, req).await;
        assert_eq!(&res[0].id, "contract.unrevealed_key");
        Ok(())
    }

    #[actix_web::test]
    async fn test_run_operation() -> Result<()> {
        let client = RollupMockClient::default();
        client.patch(|context| {
            context.set_head(Head::default()).unwrap();
            context.set_public_key("tz1KqTpEZ7Yob7QbPE4Hy4Wo8fHG8LhKxZSx", PublicKey::new(String::from("edpkuBknW28nW72KG6RoHtYW7p12T6GKc7nAbwYX5m8Wd9sDVC9yav")).unwrap()).unwrap();
            context.set_balance("tz1KqTpEZ7Yob7QbPE4Hy4Wo8fHG8LhKxZSx", Mutez::from(100000u32)).unwrap();
            context.commit().unwrap();
            Ok(())
        })?;

        let app = test::init_service(
            App::new()
                .configure(config::<RollupMockClient>)
                .app_data(Data::new(client)),
        )
        .await;

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/data/origination_payload.json");

        let mut file = File::open(path).expect("Failed to open operation payload file");
        let mut buffer: Vec<u8> = Vec::new();

        file.read_to_end(&mut buffer)
            .expect("Failed to read operation payload file");

        let req = test::TestRequest::post()
            .uri("/chains/main/blocks/head/helpers/scripts/run_operation")
            .set_payload(buffer)
            .insert_header(ContentType::json())
            .to_request();

        let res: Operation = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.hash.unwrap().into_string(), "oooHiZmTVQFVe48pqX2BqnywnH6PWDKUquYoPjtVkihLRpGQHZd");
        Ok(())
    }
}
