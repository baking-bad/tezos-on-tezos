# Kernel IO

An utility crate providing generic abstraction over the raw rollup core.

## Store

`KernelStore<Host>` is an alias for `LayeredStore<KernelBackend<Host>>`.  
Kernel backend introduces another transactional layer: data is first stored under the `/tmp` root and then you can either discard changes by calling `clear` or save them with `persist`. A typical workflow is when you use `commit/rollback` for handling individual transactions, and `persist/clear` for batches.

## Inbox

Introduces `read_inbox<Payload>()` method for dispatching both system and protocol-specific messages. You need to implement `PayloadType` trait in order to add protocol-specific parsing. You also need to provide a message prefix (typically raw rollup address) to identify messages related to your rollup in the shared inbox.
