use le_stream::derive::{FromLeBytes, ToLeBytes};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Ember duty cycle state.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum State {
    /// No Duty cycle tracking or metrics are taking place.
    TrackingOff = 0x00,
    /// Duty Cycle is tracked and has not exceeded any thresholds.
    LbtNormal = 0x01,
    /// We have exceeded the limited threshold of our total duty cycle allotment.
    LbtLimitedThresholdReached = 0x02,
    /// We have exceeded the critical threshold of our total duty cycle allotment.
    LbtCriticalThresholdReached = 0x03,
    /// We have reached the suspend limit and are blocking all outbound transmissions.
    LbtSuspendLimitReached = 0x04,
}

impl From<State> for u8 {
    fn from(state: State) -> Self {
        state as Self
    }
}

impl TryFrom<u8> for State {
    type Error = u8;

    fn try_from(duty_cycle_state: u8) -> Result<Self, Self::Error> {
        Self::from_u8(duty_cycle_state).ok_or(duty_cycle_state)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Limits {
    vendor_id: u16,
    vendor_string: [u8; 7],
}

impl Limits {
    #[must_use]
    pub const fn new(vendor_id: u16, vendor_string: [u8; 7]) -> Self {
        Self {
            vendor_id,
            vendor_string,
        }
    }

    #[must_use]
    pub const fn vendor_id(&self) -> u16 {
        self.vendor_id
    }

    #[must_use]
    pub const fn vendor_string(&self) -> &[u8; 7] {
        &self.vendor_string
    }
}
