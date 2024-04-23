use std::future::Future;

use crate::ember::NodeId;
use crate::Error;

/// Networking frames.
pub trait Networking {
    /// Convert a child index to a node ID.
    fn child_id(&self, child_index: u8) -> impl Future<Output = Result<NodeId, Error>> + Send;

    /// Clears all cached beacons that have been collected from a scan.
    fn clear_stored_beacons(&self) -> impl Future<Output = Result<(), Error>> + Send;
}
