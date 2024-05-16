use std::future::Future;

use crate::ember::child::Data;
use crate::ember::{child, network, NodeId};
use crate::error::Resolve;
use crate::frame::parameters::networking::{
    child_id, clear_stored_beacons, energy_scan_request, find_and_rejoin_network,
    find_unused_pan_id, form_network, get_child_data,
};
use crate::{Error, Transport};

/// Networking frames.
pub trait Networking {
    /// Convert a child index to a node ID.
    fn child_id(&self, child_index: u8) -> impl Future<Output = Result<NodeId, Error>> + Send;

    /// Clears all cached beacons that have been collected from a scan.
    fn clear_stored_beacons(&self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sends a ZDO energy scan request.
    ///
    /// This request may only be sent by the current network manager and must be unicast, not broadcast.
    fn energy_scan_request(
        &self,
        target: NodeId,
        scan_channels: u32,
        scan_duration: u8,
        scan_count: u16,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// The application may call this function when contact with the network has been lost.
    ///
    /// The most common usage case is when an end device can no longer communicate with its parent
    /// and wishes to find a new one. Another case is when a device has missed a Network Key update
    /// and no longer has the current Network Key.
    ///
    /// The stack will call `ezspStackStatusHandler` to indicate that the network is down,
    /// then try to re-establish contact with the network by performing an active scan,
    /// choosing a network with matching extended pan id, and sending a ZigBee network rejoin request.
    /// A second call to the `ezspStackStatusHandler` callback indicates either the success or the
    /// failure of the attempt. The process takes approximately 150 milliseconds per channel to complete.
    ///
    /// This call replaces the emberMobileNodeHasMoved API from EmberZNet 2.x,
    /// which used MAC association and consequently took half a second longer to complete.
    fn find_and_rejoin_network(
        &self,
        have_current_network_key: bool,
        channel_mask: u32,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// This function starts a series of scans which will return an available panId.
    fn find_unused_pan_id(
        &self,
        channel_mask: u32,
        duration: u8,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Forms a new network by becoming the coordinator.
    fn form_network(
        &self,
        parameters: network::Parameters,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Returns information about a child of the local node.
    fn get_child_data(&self, index: u8) -> impl Future<Output = Result<child::Data, Error>> + Send;
}

impl<T> Networking for T
where
    T: Transport,
{
    async fn child_id(&self, child_index: u8) -> Result<NodeId, Error> {
        self.communicate::<_, child_id::Response>(child_id::Command::new(child_index))
            .await
            .map(|response| response.child_id())
    }

    async fn clear_stored_beacons(&self) -> Result<(), Error> {
        self.communicate::<_, clear_stored_beacons::Response>(clear_stored_beacons::Command)
            .await
            .map(drop)
    }

    async fn energy_scan_request(
        &self,
        target: NodeId,
        scan_channels: u32,
        scan_duration: u8,
        scan_count: u16,
    ) -> Result<(), Error> {
        self.communicate::<_, energy_scan_request::Response>(energy_scan_request::Command::new(
            target,
            scan_channels,
            scan_duration,
            scan_count,
        ))
        .await?
        .resolve()
    }

    async fn find_and_rejoin_network(
        &self,
        have_current_network_key: bool,
        channel_mask: u32,
    ) -> Result<(), Error> {
        self.communicate::<_, find_and_rejoin_network::Response>(
            find_and_rejoin_network::Command::new(have_current_network_key, channel_mask),
        )
        .await?
        .resolve()
    }

    async fn find_unused_pan_id(&self, channel_mask: u32, duration: u8) -> Result<(), Error> {
        self.communicate::<_, find_unused_pan_id::Response>(find_unused_pan_id::Command::new(
            channel_mask,
            duration,
        ))
        .await?
        .resolve()
    }

    async fn form_network(&self, parameters: network::Parameters) -> Result<(), Error> {
        self.communicate::<_, form_network::Response>(form_network::Command::new(parameters))
            .await?
            .resolve()
    }

    async fn get_child_data(&self, index: u8) -> Result<Data, Error> {
        self.communicate::<_, get_child_data::Response>(get_child_data::Command::new(index))
            .await?
            .resolve()
    }
}
