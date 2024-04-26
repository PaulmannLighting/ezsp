use std::future::Future;

use crate::ember::NodeId;
use crate::error::Resolve;
use crate::frame::parameters::networking::{child_id, clear_stored_beacons, energy_scan_request};
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
}
