// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use tezos_core::types::encoded::{BlockHash, Encoded};
use tezos_smart_rollup_core::{smart_rollup_core::ReadInputMessageInfo, SmartRollupCore};
use tezos_smart_rollup_host::runtime::RuntimeError;

use crate::error::{Error, Result};

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

pub trait PayloadType: Sized {
    fn from_external_message(message: &[u8]) -> Result<Self>;
}

pub enum InboxMessage<Payload: PayloadType> {
    BeginBlock(i32),
    EndBlock(i32),
    LevelInfo(LevelInfo),
    Payload(Payload),
    NoMoreData,
    Foreign(i32),
    Unknown(i32),
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

pub fn read_inbox<Host: SmartRollupCore, Payload: PayloadType>(
    host: &mut Host,
    prefix: &[u8],
) -> Result<InboxMessage<Payload>> {
    match read_input(host) {
        Ok(Some(message)) => match message.as_ref() {
            b"\x00\x01" => Ok(InboxMessage::BeginBlock(message.level)),
            b"\x00\x02" => Ok(InboxMessage::EndBlock(message.level)),
            [b'\x00', b'\x03', info @ ..] => Ok(InboxMessage::LevelInfo(info.try_into()?)),
            [b'\x01', data @ ..] => match data.strip_prefix(prefix) {
                Some(payload) => {
                    let payload = Payload::from_external_message(payload)?;
                    Ok(InboxMessage::Payload(payload))
                }
                None => Ok(InboxMessage::Foreign(message.id)),
            },
            _ => Ok(InboxMessage::Unknown(message.id)),
        },
        Ok(None) => Ok(InboxMessage::NoMoreData),
        Err(err) => Err(err.into()),
    }
}
