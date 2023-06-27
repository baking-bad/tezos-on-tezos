// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use layered_store::{error::err_into, LayeredStore, Result, StoreBackend};
use std::collections::HashMap;
use tezos_smart_rollup_core::SmartRollupCore;
use tezos_smart_rollup_host::{
    path::{Path, RefPath},
    runtime::{Runtime, RuntimeError},
};

const TMP_PREFIX: &str = "/tmp";
const MAX_FILE_CHUNK_SIZE: usize = 2048;

macro_rules! str_to_path {
    ($key: expr) => {
        RefPath::assert_from($key.as_bytes())
    };
}

pub struct KernelBackend<'rt, Host: SmartRollupCore> {
    host: &'rt mut Host,
    saved_state: HashMap<String, bool>,
}

pub trait KernelBackendAsHost<'rt, Host: SmartRollupCore> {
    fn attach(host: &'rt mut Host) -> Self;
    fn as_host(&mut self) -> &mut Host;
}

impl<'rt, Host: SmartRollupCore> KernelBackend<'rt, Host> {
    pub fn new(host: &'rt mut Host) -> Self {
        KernelBackend {
            host,
            saved_state: HashMap::new(),
        }
    }

    pub fn persist(&mut self) -> Result<()> {
        for (key, exists) in self.saved_state.drain() {
            if exists {
                self.host
                    .store_move(
                        &str_to_path!([TMP_PREFIX, &key].concat().as_str()),
                        &str_to_path!(&key),
                    )
                    .map_err(err_into)?;
            } else {
                self.host
                    .store_delete(&str_to_path!(&key))
                    .map_err(err_into)?;
            }
        }
        Ok(())
    }
}

impl<'rt, Host: SmartRollupCore> KernelBackendAsHost<'rt, Host>
    for LayeredStore<KernelBackend<'rt, Host>>
{
    fn attach(host: &'rt mut Host) -> Self {
        Self::new(KernelBackend::new(host))
    }

    fn as_host(&mut self) -> &mut Host {
        self.as_mut().host
    }
}

// Runtime::store_read_all is available with [alloc] feature enabled,
// and it depends on the [crypto] feature we need to avoid
// because of the deps bloat and issues with building blst crate
fn store_read_all(
    host: &impl Runtime,
    path: &impl Path,
) -> std::result::Result<Vec<u8>, RuntimeError> {
    let length = Runtime::store_value_size(host, path)?;

    let mut buffer: Vec<u8> = Vec::with_capacity(length);
    let mut offset: usize = 0;

    while offset < length {
        unsafe {
            let buf_len = usize::min(offset + MAX_FILE_CHUNK_SIZE, length);
            buffer.set_len(buf_len);
        }

        let slice = &mut buffer[offset..];
        let chunk_size = host.store_read_slice(path, offset, slice)?;

        offset += chunk_size;
    }

    if offset != length {
        return Err(RuntimeError::DecodingError);
    }

    Ok(buffer)
}

impl<'rt, Host: SmartRollupCore> StoreBackend for KernelBackend<'rt, Host> {
    fn default() -> Self {
        unimplemented!()
    }

    fn log(&self, msg: &str) {
        Runtime::write_debug(self.host, msg)
    }

    fn has(&self, key: &str) -> Result<bool> {
        if let Some(has) = self.saved_state.get(key) {
            return Ok(*has);
        }

        match Runtime::store_has(self.host, &str_to_path!(&key)).map_err(err_into)? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    fn read(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let store_key = match self.saved_state.get(key) {
            Some(false) => return Ok(None),
            Some(true) => [TMP_PREFIX, &key].concat(),
            None => key.into(),
        };

        match store_read_all(self.host, &str_to_path!(&store_key)) {
            Ok(bytes) => Ok(Some(bytes)),
            Err(RuntimeError::PathNotFound) => Ok(None),
            Err(err) => Err(err_into(err)),
        }
    }

    fn write(&mut self, key: &str, val: &[u8]) -> Result<()> {
        self.host
            .store_write_all(&str_to_path!([TMP_PREFIX, key].concat().as_str()), val)
            .map_err(err_into)?;
        self.saved_state.insert(key.into(), true);
        Ok(())
    }

    fn delete(&mut self, key: &str) -> Result<()> {
        self.host
            .store_delete(&str_to_path!([TMP_PREFIX, key].concat().as_str()))
            .map_err(err_into)?;
        self.saved_state.insert(key.into(), false);
        Ok(())
    }

    fn clear(&mut self) {
        self.saved_state.clear();

        match self.host.store_delete(&str_to_path!(TMP_PREFIX)) {
            Ok(()) => {}
            Err(RuntimeError::PathNotFound) => {}
            Err(err) => panic!("Failed to clear kernel storage: {}", err),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Result;

    use tezos_smart_rollup_mock::MockHost;

    #[test]
    fn test_kernel_store() -> Result<()> {
        let mut host = MockHost::default();
        let mut store: KernelBackend<MockHost> = KernelBackend::new(&mut host);

        assert!(!store.has("/test")?);

        store.write("/test", b"deadbeef")?;
        store.persist()?;
        store.clear();

        assert!(store.has("/test")?);
        assert_eq!(b"deadbeef".to_vec(), store.read("/test")?.unwrap());

        Ok(())
    }

    #[test]
    fn store_read_all_above_max_file_chunk_size() -> Result<()> {
        // The value read is formed of 3 chunks, two of the max chunk value and
        // the last one being less than the max size.
        const PATH: RefPath<'static> = RefPath::assert_from("/a/simple/path".as_bytes());
        const VALUE_FIRST_CHUNK: [u8; MAX_FILE_CHUNK_SIZE] = [b'a'; MAX_FILE_CHUNK_SIZE];
        const VALUE_SECOND_CHUNK: [u8; MAX_FILE_CHUNK_SIZE] = [b'b'; MAX_FILE_CHUNK_SIZE];
        const VALUE_LAST_CHUNK: [u8; MAX_FILE_CHUNK_SIZE / 2] = [b'c'; MAX_FILE_CHUNK_SIZE / 2];

        let mut host = MockHost::default();

        Runtime::store_write(&mut host, &PATH, &VALUE_FIRST_CHUNK, 0)?;
        Runtime::store_write(&mut host, &PATH, &VALUE_SECOND_CHUNK, MAX_FILE_CHUNK_SIZE)?;
        Runtime::store_write(&mut host, &PATH, &VALUE_LAST_CHUNK, 2 * MAX_FILE_CHUNK_SIZE)?;

        let result = store_read_all(&host, &PATH)?;

        let mut expected: Vec<u8> = Vec::new();
        expected.extend_from_slice(&VALUE_FIRST_CHUNK);
        expected.extend_from_slice(&VALUE_SECOND_CHUNK);
        expected.extend_from_slice(&VALUE_LAST_CHUNK);

        assert_eq!(expected, result);
        Ok(())
    }
}
