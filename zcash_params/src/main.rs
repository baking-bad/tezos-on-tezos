// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use bellman::groth16::VerifyingKey;
use bls12_381::Bls12;
use std::fs::File;
use std::path::Path;
use zcash_proofs::{download_sapling_parameters, load_parameters};

fn save_vk(vk: &VerifyingKey<Bls12>, path: &Path) {
    let mut writer = File::create(path).expect("Failed to create key file");
    vk.write(&mut writer).expect("Failed to serialize vk");
}

fn main() {
    let target_dir = Path::new("./sapling_proto/src/keys");
    assert!(target_dir.exists() && target_dir.is_dir());

    let params_paths =
        download_sapling_parameters(Some(60)).expect("Failed to download sapling parameters");

    let params = load_parameters(&params_paths.spend, &params_paths.output, None);
    save_vk(&params.spend_params.vk, &target_dir.join("spend.bin"));
    save_vk(&params.output_params.vk, &target_dir.join("output.bin"));
}
