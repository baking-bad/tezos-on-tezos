use host::{
    input::Input,
    rollup_core::MAX_INPUT_MESSAGE_SIZE,
    runtime::Runtime
};
use proto::producer::types::{SignedOperation, OperationHash, UnsignedOperation, Signature, Encoded};

use crate::{error::Result, parsing_error};

const SIGNATURE_SIZE: usize = 64;

pub enum InboxMessage {
    BeginBlock(i32),
    EndBlock(i32),
    LevelInfo(Vec<u8>),
    L2Operation {
        hash: OperationHash,
        opg: SignedOperation
    },
    NoMoreData,
    Unknown(i32)
}

pub fn parse_l2_operation<'a>(payload: &'a [u8]) -> Result<InboxMessage> {
    if payload.len() <= SIGNATURE_SIZE {
        return parsing_error!("L2 operation payload is too short");
    }
    let unsigned_op = UnsignedOperation::from_forged_bytes(&payload[..payload.len() - SIGNATURE_SIZE])?;  
    let signature = Signature::from_bytes(&payload[payload.len() - SIGNATURE_SIZE..])?;
    let hash = SignedOperation::operation_hash(payload)?;
    Ok(InboxMessage::L2Operation {
        hash,
        opg: SignedOperation::from(unsigned_op, signature)
    })
}

pub fn read_inbox(host: &mut impl Runtime) -> Result<InboxMessage> {
    match host.read_input(MAX_INPUT_MESSAGE_SIZE) {
        Ok(Some(Input::Message(message))) => {
            match message.as_ref() {
                b"\x00\x01" => Ok(InboxMessage::BeginBlock(message.level)),
                b"\x00\x02" => Ok(InboxMessage::EndBlock(message.level)),
                [b'\x00', info @ ..] => Ok(InboxMessage::LevelInfo(info.to_vec())),
                [b'\x01', payload @ ..] => parse_l2_operation(payload),  // TODO: add chain_id prefix
                _ => Ok(InboxMessage::Unknown(message.id))
            }
        },
        Ok(Some(Input::Slot(_message))) => todo!("handle slot message"),
        Ok(None) => Ok(InboxMessage::NoMoreData),
        Err(err) => Err(err.into())
    }
}