// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use kernel_io::{error::err_into, inbox::PayloadType};
use tezos_core::types::encoded::{Encoded, OperationHash, Signature};
use tezos_operation::operations::{SignedOperation, UnsignedOperation};

use crate::error::{Error, Result};

const SIGNATURE_SIZE: usize = 64;

pub fn parse_l2_operation<'a>(bytes: &'a [u8]) -> Result<(OperationHash, SignedOperation)> {
    if bytes.len() <= SIGNATURE_SIZE {
        return Err(Error::UnexpectedL2OperationLength {
            length: bytes.len(),
        });
    }

    let unsigned_op = UnsignedOperation::from_forged_bytes(&bytes[..bytes.len() - SIGNATURE_SIZE])?;
    let signature = Signature::from_bytes(&bytes[bytes.len() - SIGNATURE_SIZE..])?;
    let hash = SignedOperation::operation_hash(bytes)?;
    let opg = SignedOperation::from(unsigned_op, signature);

    Ok((hash, opg))
}

pub enum TezosPayload {
    Operation {
        hash: OperationHash,
        opg: SignedOperation,
    },
}

impl PayloadType for TezosPayload {
    fn from_external_message(message: &[u8]) -> kernel_io::Result<Self> {
        let (hash, opg) = parse_l2_operation(message).map_err(err_into)?;
        Ok(TezosPayload::Operation { hash, opg })
    }
}
