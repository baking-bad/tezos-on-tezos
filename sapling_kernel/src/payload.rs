// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use kernel_io::{error::err_into, inbox::PayloadType};
use sapling_proto::types::SaplingTransaction;

pub enum SaplingPayload {
    Transaction(SaplingTransaction),
}

impl PayloadType for SaplingPayload {
    fn from_external_message(message: &[u8]) -> kernel_io::Result<Self> {
        let tx = SaplingTransaction::try_from(message).map_err(err_into)?;
        Ok(SaplingPayload::Transaction(tx))
    }
}
