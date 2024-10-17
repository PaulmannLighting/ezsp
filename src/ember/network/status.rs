use num_derive::FromPrimitive;

/// Ember network status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Status {
    /// The node is not associated with a network in any way.
    NoNetwork = 0x00,
    /// The node is currently attempting to join a network.
    JoiningNetwork = 0x01,
    /// The node is joined to a network.
    JoinedNetwork = 0x02,
    /// The node is an end device joined to a network but its parent is not responding.
    JoinedNetworkNoParent = 0x03,
    /// The node is in the process of leaving its current network.
    LeavingNetwork = 0x04,
}

impl From<Status> for u8 {
    fn from(status: Status) -> Self {
        status as Self
    }
}
