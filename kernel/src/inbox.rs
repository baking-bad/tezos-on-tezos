use tezos_core::types::encoded::{BlockHash, Encoded, OperationHash, Signature};
use tezos_operation::operations::{SignedOperation, UnsignedOperation};
use tezos_smart_rollup_host::runtime::Runtime;

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
    BeginBlock(u32),
    EndBlock(u32),
    LevelInfo(LevelInfo),
    L2Operation {
        hash: OperationHash,
        opg: SignedOperation,
    },
    NoMoreData,
    Unknown(u32),
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

pub fn read_inbox(host: &mut impl Runtime, chain_prefix: &[u8]) -> Result<InboxMessage> {
    match host.read_input() {
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
