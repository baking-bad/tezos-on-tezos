pub mod blocks;
pub mod context;
pub mod contracts;
pub mod helpers;
pub mod operations;
pub mod shell;

use crate::rollup::{RollupClient, TezosFacade, TezosHelpers};
use crate::services::{
    blocks::{block, block_hash, block_header, block_metadata, block_protocols, live_blocks},
    context::{big_map_value, big_map_value_normalized, constants, delegate, delegates},
    contracts::{
        contract, contract_balance, contract_counter, contract_delegate, contract_entrypoints,
        contract_public_key, contract_script, contract_script_normalized, contract_storage,
    },
    helpers::{forge_operation, preapply_operations, run_operation, simulate_operation},
    operations::{
        operation, operation_hash, operation_hash_list, operation_hash_list_list, operation_list,
        operation_list_list,
    },
    shell::{chain_id, inject_operation, is_bootstrapped, pending_operations, version},
};
use actix_web::web::{get, post, ServiceConfig};

use self::blocks::{block_header_shell, bootstrap_info};

#[macro_export]
macro_rules! json_response {
    ($value: expr) => {
        actix_web::HttpResponse::build(actix_web::http::StatusCode::OK).json($value)
    };
}

pub fn config<T: RollupClient + TezosFacade + TezosHelpers + 'static>(cfg: &mut ServiceConfig) {
    cfg.route("/version", get().to(version::<T>))
        .route("/chains/main/chain_id", get().to(chain_id::<T>))
        .route(
            "/chains/main/is_bootstrapped",
            get().to(is_bootstrapped::<T>),
        )
        .route(
            "/chains/main/mempool/pending_operations",
            get().to(pending_operations::<T>),
        )
        .route("/injection/operation", post().to(inject_operation::<T>))
        .route(
            "/chains/main/blocks/{block_id}/helpers/scripts/run_operation",
            post().to(run_operation::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/helpers/scripts/simulate_operation",
            post().to(simulate_operation::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/helpers/forge/operations",
            post().to(forge_operation::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/helpers/preapply/operations",
            post().to(preapply_operations::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/hash",
            get().to(block_hash::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/header",
            get().to(block_header::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/header/shell",
            get().to(block_header_shell::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/metadata",
            get().to(block_metadata::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/protocols",
            get().to(block_protocols::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/live_blocks",
            get().to(live_blocks::<T>),
        )
        .route("/chains/main/blocks/{block_id}", get().to(block::<T>))
        .route("/monitor/bootstrapped", get().to(bootstrap_info::<T>))
        .route(
            "/chains/main/blocks/{block_id}/context/delegates",
            get().to(delegates),
        )
        .route(
            "/chains/main/blocks/{block_id}/context/delegates/{delegate_id}",
            get().to(delegate),
        )
        .route(
            "/chains/main/blocks/{block_id}/context/constants",
            get().to(constants),
        )
        .route(
            "/chains/main/blocks/{block_id}/context/big_maps/{big_map_id}/{key_hash}",
            get().to(big_map_value::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/context/big_maps/{big_map_id}/{key_hash}/normalized",
            post().to(big_map_value_normalized::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/context/contracts/{contract_id}/manager_key",
            get().to(contract_public_key::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/context/contracts/{contract_id}/balance",
            get().to(contract_balance::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/context/contracts/{contract_id}/counter",
            get().to(contract_counter::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/context/contracts/{contract_id}/delegate",
            get().to(contract_delegate::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/context/contracts/{contract_id}/storage",
            get().to(contract_storage::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/context/contracts/{contract_id}/script",
            get().to(contract_script::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/context/contracts/{contract_id}/script/normalized",
            post().to(contract_script_normalized::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/context/contracts/{contract_id}/entrypoints",
            get().to(contract_entrypoints::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/context/contracts/{contract_id}",
            get().to(contract::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/operations/{pass}/{index}",
            get().to(operation::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/operations/{pass}",
            get().to(operation_list::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/operations",
            get().to(operation_list_list::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/operation_hashes/{pass}/{index}",
            get().to(operation_hash::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/operation_hashes/{pass}",
            get().to(operation_hash_list::<T>),
        )
        .route(
            "/chains/main/blocks/{block_id}/operation_hashes",
            get().to(operation_hash_list_list::<T>),
        );
}
