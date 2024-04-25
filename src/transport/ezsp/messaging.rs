use std::future::Future;

use crate::ember::beacon::ClassificationParams;
use crate::ember::{Eui64, NodeId};
use crate::error::Resolve;
use crate::frame::parameters::messaging::{
    address_table_entry_is_active, get_address_table_remote_eui64,
    get_address_table_remote_node_id, get_beacon_classification_params, get_extended_timeout,
};
use crate::{Error, Transport};

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
    ) -> impl Future<Output = Result<NodeId, Error>> + Send;

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

impl<T> Messaging for T
where
    T: Transport,
{
    async fn address_table_entry_is_active(&self, address_table_index: u8) -> Result<bool, Error> {
        self.communicate::<_, address_table_entry_is_active::Response>(
            address_table_entry_is_active::Command::new(address_table_index),
        )
        .await
        .map(|response| response.active())
    }

    async fn get_address_table_remote_eui64(
        &self,
        address_table_index: u8,
    ) -> Result<Eui64, Error> {
        self.communicate::<_, get_address_table_remote_eui64::Response>(
            get_address_table_remote_eui64::Command::new(address_table_index),
        )
        .await
        .map(|response| response.eui64())
    }

    async fn get_address_table_remote_node_id(
        &self,
        address_table_index: u8,
    ) -> Result<NodeId, Error> {
        self.communicate::<_, get_address_table_remote_node_id::Response>(
            get_address_table_remote_node_id::Command::new(address_table_index),
        )
        .await
        .map(|response| response.node_id())
    }

    async fn get_beacon_classification_params(&self) -> Result<ClassificationParams, Error> {
        self.communicate::<_, get_beacon_classification_params::Response>(
            get_beacon_classification_params::Command,
        )
        .await?
        .resolve()
    }

    async fn get_extended_timeout(&self, remote_eui64: Eui64) -> Result<bool, Error> {
        self.communicate::<_, get_extended_timeout::Response>(get_extended_timeout::Command::new(
            remote_eui64,
        ))
        .await
        .map(|response| response.extended_timeout())
    }
}
