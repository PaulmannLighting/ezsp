use std::future::Future;

use crate::ember::beacon::ClassificationParams;
use crate::ember::Eui64;
use crate::Error;

pub trait Messaging {
    /// Indicates whether any messages are currently being sent using this address table entry.
    /// Note that this function does not indicate whether the address table entry is unused.
    /// To determine whether an address table entry is unused, check the remote node ID.
    /// The remote node ID will have the value `EMBER_TABLE_ENTRY_UNUSED_NODE_ID`
    /// when the address table entry is not in use.
    fn address_table_entry_is_active(
        &self,
        address_table_index: u8,
    ) -> impl Future<Output = Result<bool, Error>> + Send;

    /// Gets the EUI64 of an address table entry.
    fn get_address_table_remote_eui64(
        &self,
        address_table_index: u8,
    ) -> impl Future<Output = Result<Eui64, Error>> + Send;

    /// Gets the short ID of an address table entry.
    fn get_address_table_remote_node_id(
        &self,
        address_table_index: u8,
    ) -> impl Future<Output = Result<Eui64, Error>> + Send;

    /// Gets the priority masks and related variables for choosing the best beacon.
    fn get_beacon_classification_params(
        &self,
    ) -> impl Future<Output = Result<ClassificationParams, Error>> + Send;

    /// Indicates whether the stack will extend the normal interval between retransmissions
    /// of a retried unicast message by `EMBER_INDIRECT_TRANSMISSION_TIMEOUT`.
    fn get_extended_timeout(
        &self,
        remote_eui64: Eui64,
    ) -> impl Future<Output = Result<bool, Error>> + Send;
}
