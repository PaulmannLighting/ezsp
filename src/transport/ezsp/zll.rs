use crate::Transport;

/// The `Zll` trait provides an interface for the Zigbee Light Link (ZLL) protocol.
pub trait Zll {}

impl<T> Zll for T where T: Transport {}
