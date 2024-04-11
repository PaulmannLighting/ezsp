use crate::ember::NodeId;
use crate::{Error, Transport};
use std::future::Future;

/// Networking frames.
pub trait Networking: Transport {
    fn child_id(&self, child_index: u8) -> impl Future<Output = Result<NodeId, Error>> + Send;
}
