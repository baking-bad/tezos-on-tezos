// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use tezos_core::types::encoded::{BlockHash, Encoded, OperationHash, Signature};
use tezos_operation::operations::{SignedOperation, UnsignedOperation};
use tezos_smart_rollup_core::{smart_rollup_core::ReadInputMessageInfo, SmartRollupCore};
use tezos_smart_rollup_host::runtime::RuntimeError;

use crate::error::{Error, Result};

const CHAIN_ID_SIZE: usize = 4;
const SIGNATURE_SIZE: usize = 64;

#[derive(Debug, Clone)]
pub struct LevelInfo {
    pub predecessor_timestamp: i64,
    pub predecessor: BlockHash,
}

impl TryFrom<&[u8]> for LevelInfo {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() != 8 + 32 {
            return Err(Error::UnexpectedLevelInfoLength {
                length: value.len(),
            });
        }
        let predecessor_timestamp = i64::from_be_bytes([
            value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7],
        ]);
        let predecessor = BlockHash::from_bytes(&value[8..])?;
        Ok(Self {
            predecessor_timestamp,
            predecessor,
        })
    }
}

pub enum InboxMessage {
    BeginBlock(i32),
    EndBlock(i32),
    LevelInfo(LevelInfo),
    L2Operation {
        hash: OperationHash,
        opg: SignedOperation,
    },
    NoMoreData,
    Unknown(i32),
}

pub fn parse_l2_operation<'a>(payload: &'a [u8], chain_prefix: &[u8]) -> Result<InboxMessage> {
    if payload.len() <= CHAIN_ID_SIZE + SIGNATURE_SIZE {
        return Err(Error::UnexpectedL2OperationLength {
            length: payload.len(),
        });
    }

    if payload[..CHAIN_ID_SIZE] != *chain_prefix {
        return Err(Error::UnexpectedL2OperationPrefix);
    }

    let unsigned_op = UnsignedOperation::from_forged_bytes(
        &payload[CHAIN_ID_SIZE..payload.len() - SIGNATURE_SIZE],
    )?;
    let signature = Signature::from_bytes(&payload[payload.len() - SIGNATURE_SIZE..])?;
    let hash = SignedOperation::operation_hash(payload)?;
    Ok(InboxMessage::L2Operation {
        hash,
        opg: SignedOperation::from(unsigned_op, signature),
    })
}

#[derive(Clone, Debug)]
pub struct Message {
    pub level: i32,
    pub id: i32,
    pub payload: Vec<u8>,
}

impl AsRef<[u8]> for Message {
    fn as_ref(&self) -> &[u8] {
        self.payload.as_slice()
    }
}

pub fn read_input<Host: SmartRollupCore>(
    host: &mut Host,
) -> std::result::Result<Option<Message>, RuntimeError> {
    use core::mem::MaybeUninit;
    use tezos_smart_rollup_core::MAX_INPUT_MESSAGE_SIZE;

    let mut buffer = Vec::with_capacity(MAX_INPUT_MESSAGE_SIZE);

    let mut message_info = MaybeUninit::<ReadInputMessageInfo>::uninit();

    let bytes_read = unsafe {
        SmartRollupCore::read_input(
            host,
            message_info.as_mut_ptr(),
            buffer.as_mut_ptr(),
            MAX_INPUT_MESSAGE_SIZE,
        )
    };

    let bytes_read = match tezos_smart_rollup_host::Error::wrap(bytes_read) {
        Ok(0) => return Ok(None),
        Ok(size) => size,
        Err(e) => return Err(RuntimeError::HostErr(e)),
    };

    let ReadInputMessageInfo { level, id } = unsafe {
        buffer.set_len(bytes_read);
        message_info.assume_init()
    };

    let input = Message {
        level,
        id,
        payload: buffer,
    };

    Ok(Some(input))
}

pub fn read_inbox<Host: SmartRollupCore>(
    host: &mut Host,
    chain_prefix: &[u8],
) -> Result<InboxMessage> {
    match read_input(host) {
        Ok(Some(message)) => match message.as_ref() {
            b"\x00\x01" => Ok(InboxMessage::BeginBlock(message.level)),
            b"\x00\x02" => Ok(InboxMessage::EndBlock(message.level)),
            [b'\x00', b'\x03', info @ ..] => Ok(InboxMessage::LevelInfo(info.try_into()?)),
            [b'\x01', payload @ ..] => parse_l2_operation(payload, chain_prefix),
            _ => Ok(InboxMessage::Unknown(message.id)),
        },
        Ok(None) => Ok(InboxMessage::NoMoreData),
        Err(err) => Err(err.into()),
    }
}
