use crate::{Error, Transport};
use std::future::Future;

pub trait Messaging: Transport {
    /// Indicates whether any messages are currently being sent using this address table entry.
    /// Note that this function does not indicate whether the address table entry is unused.
    /// To determine whether an address table entry is unused, check the remote node ID.
    /// The remote node ID will have the value `EMBER_TABLE_ENTRY_UNUSED_NODE_ID`
    /// when the address table entry is not in use.
    fn address_table_entry_is_active(
        &self,
        address_table_index: u8,
    ) -> impl Future<Output = Result<bool, Error>> + Send;
}
