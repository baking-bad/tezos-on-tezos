use actix_web::{
    http::header,
    web::{Data, Path},
    HttpResponse, Responder, Result,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tezos_core::types::encoded::{BlockHash, ContextHash, OperationListListHash};
use tokio_stream::wrappers::ReceiverStream;

use crate::{
    json_response,
    rollup::{BlockId, TezosFacade},
};

pub mod rfc3339_timestamp;

#[derive(Serialize, Deserialize)]
pub struct BootstrapInfo {
    pub hash: BlockHash,
    #[serde(with = "rfc3339_timestamp")]
    pub timestamp: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct HeaderShell {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<BlockHash>,
    pub level: i32,
    pub proto: u8,
    pub predecessor: BlockHash,
    #[serde(with = "rfc3339_timestamp")]
    pub timestamp: NaiveDateTime,
    pub validation_pass: u8,
    pub operations_hash: OperationListListHash,
    pub fitness: Vec<String>,
    pub context: ContextHash,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_data: Option<String>,
}

async fn get_block_id<T: TezosFacade>(client: &Data<T>, block_id: &str) -> Result<BlockId> {
    let value = match block_id.try_into() {
        Ok(block_id) => block_id,
        Err(_) => {
            let hash = &block_id[0..51];
            let offset_str = block_id.trim_start_matches(format!("{}{}", hash, "~").as_str());
            let offset = i32::from_str_radix(offset_str, 10).unwrap();
            let header = client.get_block_header(&hash.try_into()?).await?;
            let target_level = header.level - offset;
            BlockId::Level(target_level.try_into().unwrap())
        }
    };
    Ok(value)
}

pub async fn block_hash<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String,)>,
) -> Result<impl Responder> {
    let block_id = get_block_id(&client, path.0.as_str()).await?;
    let value = client.get_block_hash(&block_id).await?;
    Ok(json_response!(value))
}

pub async fn block_header<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String,)>,
) -> Result<impl Responder> {
    let block_id = get_block_id(&client, path.0.as_str()).await?;
    let value = client.get_block_header(&block_id).await?;
    Ok(json_response!(value))
}

pub async fn block_header_shell<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String,)>,
) -> Result<impl Responder> {
    let block_id = get_block_id(&client, path.0.as_str()).await?;
    let full_header = client.get_block_header(&block_id).await?;
    let value = HeaderShell {
        hash: Option::None,
        level: full_header.level,
        proto: full_header.proto,
        predecessor: full_header.predecessor,
        timestamp: full_header.timestamp,
        validation_pass: full_header.validation_pass,
        operations_hash: full_header.operations_hash,
        fitness: full_header.fitness,
        context: full_header.context,
        protocol_data: Option::None,
    };
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

pub async fn bootstrap_info<T: TezosFacade>(client: Data<T>) -> Result<impl Responder> {
    let header = client.get_block_header(&BlockId::Head).await?;
    let value = BootstrapInfo {
        hash: header.hash,
        timestamp: header.timestamp,
    };
    Ok(json_response!(value))
}

pub async fn heads_main<T: TezosFacade>(client: Data<T>) -> Result<impl Responder> {
    let rx = client.get_heads_main_channel().await.unwrap();
    let body_stream = ReceiverStream::new(rx);

    let response = HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "application/json"))
        .streaming(body_stream);

    Ok(response)
}

#[cfg(test)]
mod test {
    use actix_web::{test, web::Data, App};
    use tezos_core::types::encoded::{BlockHash, Encoded};
    use tezos_ctx::{BatchHeader, BatchReceipt, ExecutorContext, Head};
    use tezos_rpc::models::block::FullHeader;

    use crate::{
        rollup::mock_client::RollupMockClient,
        services::{
            blocks::{BootstrapInfo, HeaderShell},
            config,
        },
        Result,
    };

    fn get_test_batch_receipt() -> BatchReceipt {
        BatchReceipt {
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
        }
    }

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
            context.set_batch_receipt(get_test_batch_receipt()).unwrap();
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

    #[actix_web::test]
    async fn test_block_header_shell() -> Result<()> {
        let client = RollupMockClient::default();
        client.patch(|context| {
            context.set_head(Head::default()).unwrap();
            context.set_batch_receipt(get_test_batch_receipt()).unwrap();
            Ok(())
        })?;

        let app = test::init_service(
            App::new()
                .configure(config::<RollupMockClient>)
                .app_data(Data::new(client)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/chains/main/blocks/head/header/shell")
            .to_request();
        let res: HeaderShell = test::call_and_read_body_json(&app, req).await;

        assert_eq!(3113764, res.level);
        assert_eq!(0, res.proto);
        assert_eq!(
            "BM4iF1PGVN74h1kvqUtY26boVKpZuJFvpQRN34JLYSkQ9G3jBnn",
            res.predecessor.value()
        );
        assert_eq!(4, res.validation_pass);
        assert_eq!(
            "LLoZzirJsJexRFU6yznwPxSfETprh8Yd5scW1DXfFAwoGmcvQJteE",
            res.operations_hash.value()
        );
        assert_eq!(
            "CoVnr5Sy57UkHt1Aqmw62KyLUYxVmKCyp45HY7MXW8sFemT3Uf6i",
            res.context.value()
        );
        Ok(())
    }

    #[actix_web::test]
    async fn test_bootstrap_info() -> Result<()> {
        let client = RollupMockClient::default();
        client.patch(|context| {
            context.set_head(Head::default()).unwrap();
            context.set_batch_receipt(get_test_batch_receipt()).unwrap();
            Ok(())
        })?;

        let app = test::init_service(
            App::new()
                .configure(config::<RollupMockClient>)
                .app_data(Data::new(client)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/monitor/bootstrapped")
            .to_request();

        let res: BootstrapInfo = test::call_and_read_body_json(&app, req).await;
        assert_eq!(
            "BKiHLREqU3JkXfzEDYAkmmfX48gBDtYhMrpA98s7Aq4SzbUAB6M",
            res.hash.value()
        );
        Ok(())
    }
}
