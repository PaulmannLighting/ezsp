use le_stream::derive::{FromLeBytes, ToLeBytes};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum State {
    TrackingOff = 0x00,
    LbtNormal = 0x01,
    LbtLimitedThresholdReached = 0x02,
    LbtCriticalThresholdReached = 0x03,
    LbtSuspendLimitReached = 0x04,
}

impl From<State> for u8 {
    fn from(state: State) -> Self {
        state
            .to_u8()
            .expect("State should always be convertible to u8.")
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
    pub fn new(vendor_id: u16, vendor_string: [u8; 7]) -> Self {
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
