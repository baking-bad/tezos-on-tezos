// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use actix_web::{
    web::{Data, Path},
    Responder, Result,
};

use crate::{json_response, rollup::TezosFacade};

pub async fn block_hash<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String,)>,
) -> Result<impl Responder> {
    let value = client.get_block_hash(&path.0.as_str().try_into()?).await?;
    Ok(json_response!(value))
}

pub async fn block_header<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String,)>,
) -> Result<impl Responder> {
    let value = client
        .get_block_header(&path.0.as_str().try_into()?)
        .await?;
    Ok(json_response!(value))
}

pub async fn block_metadata<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String,)>,
) -> Result<impl Responder> {
    let value = client
        .get_block_metadata(&path.0.as_str().try_into()?)
        .await?;
    Ok(json_response!(value))
}

pub async fn block_protocols<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String,)>,
) -> Result<impl Responder> {
    let value = client
        .get_block_protocols(&path.0.as_str().try_into()?)
        .await?;
    Ok(json_response!(value))
}

pub async fn live_blocks<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String,)>,
) -> Result<impl Responder> {
    let value = client.get_live_blocks(&path.0.as_str().try_into()?).await?;
    Ok(json_response!(value))
}

pub async fn block<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String,)>,
) -> Result<impl Responder> {
    let value = client.get_block(&path.0.as_str().try_into()?).await?;
    Ok(json_response!(value))
}

#[cfg(test)]
mod test {
    use actix_web::{test, web::Data, App};
    use tezos_core::types::encoded::{BlockHash, Encoded};
    use tezos_proto::context::{
        batch::{BatchHeader, BatchReceipt},
        head::Head,
        TezosContext,
    };
    use tezos_rpc::models::block::FullHeader;

    use crate::{rollup::mock_client::RollupMockClient, services::config, Result};

    #[actix_web::test]
    async fn test_block_hash() -> Result<()> {
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
            .uri("/chains/main/blocks/head/hash")
            .to_request();
        let res: BlockHash = test::call_and_read_body_json(&app, req).await;
        assert_eq!(
            "BKiHLREqU3JkXfzEDYAkmmfX48gBDtYhMrpA98s7Aq4SzbUAB6M",
            res.value()
        );
        Ok(())
    }

    #[actix_web::test]
    async fn test_block_header() -> Result<()> {
        let client = RollupMockClient::default();
        client.patch(|context| {
            context.set_head(Head::default()).unwrap();
            context
                .set_batch_receipt(BatchReceipt {
                    balance_updates: None,
                    chain_id: "NetXdQprcVkpaWU".try_into().unwrap(),
                    hash: "BKiHLREqU3JkXfzEDYAkmmfX48gBDtYhMrpA98s7Aq4SzbUAB6M"
                        .try_into()
                        .unwrap(), // head::default
                    protocol: "PtLimaPtLMwfNinJi9rCfDPWea8dFgTZ1MeJ9f1m2SRic6ayiwW"
                        .try_into()
                        .unwrap(),
                    header: BatchHeader {
                        predecessor: "BM4iF1PGVN74h1kvqUtY26boVKpZuJFvpQRN34JLYSkQ9G3jBnn"
                            .try_into()
                            .unwrap(),
                        level: 3113764,
                        timestamp: 1675429049,
                        operations_hash: "LLoZzirJsJexRFU6yznwPxSfETprh8Yd5scW1DXfFAwoGmcvQJteE"
                            .try_into()
                            .unwrap(),
                        payload_hash: "vh2tsz2UVT8kVJzgye4MpngkRvJJLgnNhZjUgn9oURpB1PYpyrFK"
                            .try_into()
                            .unwrap(),
                        context: "CoVnr5Sy57UkHt1Aqmw62KyLUYxVmKCyp45HY7MXW8sFemT3Uf6i"
                            .try_into()
                            .unwrap(),
                    },
                })
                .unwrap();
            Ok(())
        })?;

        let app = test::init_service(
            App::new()
                .configure(config::<RollupMockClient>)
                .app_data(Data::new(client)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/chains/main/blocks/head/header")
            .to_request();
        let res: FullHeader = test::call_and_read_body_json(&app, req).await;
        assert_eq!(
            "BM4iF1PGVN74h1kvqUtY26boVKpZuJFvpQRN34JLYSkQ9G3jBnn",
            res.predecessor.value()
        );
        assert_eq!("NetXdQprcVkpaWU", res.chain_id.value());
        Ok(())
    }
}
