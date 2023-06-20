use actix_web::{
    http::StatusCode,
    web::{Data, Path},
    HttpResponse, Responder, Result,
};
use tezos_core::types::encoded::{Address, ContractAddress, ImplicitAddress};

use crate::{json_response, rollup::TezosFacade, Error};

pub async fn contract_balance<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String, String)>,
) -> Result<impl Responder> {
    let address: Address = path.1.as_str().try_into().map_err(Error::from)?;
    let value = client
        .get_contract_balance(&path.0.as_str().try_into()?, &address)
        .await?;
    Ok(json_response!(value))
}

pub async fn contract_counter<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String, String)>,
) -> Result<impl Responder> {
    let address: ImplicitAddress = path.1.as_str().try_into().map_err(Error::from)?;
    let value = client
        .get_contract_counter(&path.0.as_str().try_into()?, &address)
        .await?;
    Ok(json_response!(value))
}

pub async fn contract_public_key<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String, String)>,
) -> Result<impl Responder> {
    let address: ImplicitAddress = path.1.as_str().try_into().map_err(Error::from)?;
    let value = client
        .get_contract_public_key(&path.0.as_str().try_into()?, &address)
        .await?;
    Ok(json_response!(value))
}

pub async fn contract_delegate<T: TezosFacade>(
    _client: Data<T>,
    _path: Path<(String, String)>,
) -> Result<impl Responder> {
    Ok(HttpResponse::build(StatusCode::NOT_FOUND).finish())
}

pub async fn contract_storage<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String, String)>,
) -> Result<impl Responder> {
    let address: ContractAddress = path.1.as_str().try_into().map_err(Error::from)?;
    let value = client
        .get_contract_storage(&path.0.as_str().try_into()?, &address)
        .await?;
    Ok(json_response!(value))
}

pub async fn contract_script<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String, String)>,
) -> Result<impl Responder> {
    let address: ContractAddress = path.1.as_str().try_into().map_err(Error::from)?;
    let value = client
        .get_contract_script(&path.0.as_str().try_into()?, &address)
        .await?;
    Ok(json_response!(value))
}

pub async fn contract_script_normalized<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String, String)>,
) -> Result<impl Responder> {
    // TODO: handle unparsing mode https://gitlab.com/tezos/tezos/-/blob/master/docs/api/mumbai-openapi.json
    contract_script(client, path).await
}

pub async fn contract_entrypoints<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String, String)>,
) -> Result<impl Responder> {
    let address: ContractAddress = path.1.as_str().try_into().map_err(Error::from)?;
    let value = client
        .get_contract_entrypoints(&path.0.as_str().try_into()?, &address)
        .await?;
    Ok(json_response!(value))
}

pub async fn contract<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String, String)>,
) -> Result<impl Responder> {
    let address: Address = path.1.as_str().try_into().map_err(Error::from)?;
    let value = client
        .get_contract(&path.0.as_str().try_into()?, &address)
        .await?;
    Ok(json_response!(value))
}

#[cfg(test)]
mod test {
    use actix_web::{test, web::Data, App};
    use tezos_core::types::{encoded::PublicKey, mutez::Mutez, number::Nat};
    use tezos_proto::context::{TezosContext, head::Head};
    use tezos_rpc::models::contract::ContractInfo;

    use crate::{rollup::mock_client::RollupMockClient, services::config, Result};

    #[actix_web::test]
    async fn test_non_existent_contract() -> Result<()> {
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

        let req = test::TestRequest::get()
            .uri("/chains/main/blocks/head/context/contracts/tz1N2386URdGRJDbMuEEn757Y5HBPg5XwtmQ")
            .to_request();
        let res: ContractInfo = test::call_and_read_body_json(&app, req).await;
        assert_eq!(Mutez::from(0u32), res.balance);
        assert_eq!(Some(Nat::from("0").unwrap()), res.counter);
        Ok(())
    }

    #[actix_web::test]
    async fn test_non_existent_manager_key() -> Result<()> {
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

        let req = test::TestRequest::get()
            .uri("/chains/main/blocks/head/context/contracts/tz1N2386URdGRJDbMuEEn757Y5HBPg5XwtmQ/manager_key")
            .to_request();
        let res: Option<PublicKey> = test::call_and_read_body_json(&app, req).await;
        assert!(res.is_none());
        Ok(())
    }
}
