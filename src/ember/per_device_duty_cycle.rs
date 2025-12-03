use le_stream::{FromLeStream, ToLeStream};

use crate::ember::NodeId;

/// A structure containing per device overall duty cycle consumed (up to the suspend limit).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct PerDeviceDutyCycle {
    node_id: NodeId,
    duty_cycle_consumed: u16,
}

impl PerDeviceDutyCycle {
    /// Creates a new `PerDeviceDutyCycle`.
    #[must_use]
    pub const fn new(node_id: NodeId, duty_cycle_consumed: u16) -> Self {
        Self {
            node_id,
            duty_cycle_consumed,
        }
    }

    /// Node ID of device whose duty cycle is reported.
    #[must_use]
    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }

    /// Amount of overall duty cycle consumed (up to suspend limit).
    #[must_use]
    pub const fn duty_cycle_consumed(&self) -> u16 {
        self.duty_cycle_consumed
    }
}
