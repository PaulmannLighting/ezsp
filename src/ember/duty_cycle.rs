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
    fn from(duty_cycle_state: State) -> Self {
        duty_cycle_state
            .to_u8()
            .expect("could not convert DutyCycleState to u8")
    }
}

impl TryFrom<u8> for State {
    type Error = u8;

    fn try_from(duty_cycle_state: u8) -> Result<Self, Self::Error> {
        Self::from_u8(duty_cycle_state).ok_or(duty_cycle_state)
    }
}
