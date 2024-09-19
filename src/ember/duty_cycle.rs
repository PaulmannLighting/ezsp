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

/// A structure containing duty cycle limit configurations.
///
/// All limits are absolute, and are required to be as follows:
///
/// `susp_limit` > `crit_thresh` > `limit_thresh`
///
/// For example:
///
/// `susp_limit = 250` (2.5%), `crit_thresh = 180` (1.8%), `limit_thresh = 100` (1.00%).
///
/// See [EmberDutyCycleLimits Struct Reference](https://docs.silabs.com/zigbee/6.6/em35x/structEmberDutyCycleLimits)
/// for more information.
#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Limits {
    crit_thresh: u16,
    limit_thresh: u16,
    susp_limit: u16,
}

impl Limits {
    /// Attempt to create a new duty cycle limit configuration, checking the limits.
    #[must_use]
    pub const fn try_new(crit_thresh: u16, limit_thresh: u16, susp_limit: u16) -> Option<Self> {
        if susp_limit > crit_thresh && crit_thresh > limit_thresh {
            #[allow(unsafe_code)]
            // SAFETY: We checked the limit constraints in the line above.
            Some(unsafe { Self::new_unchecked(crit_thresh, limit_thresh, susp_limit) })
        } else {
            None
        }
    }

    /// Create a new duty cycle limit configuration without checking the limits.
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(
        crit_thresh: u16,
        limit_thresh: u16,
        susp_limit: u16,
    ) -> Self {
        Self {
            crit_thresh,
            limit_thresh,
            susp_limit,
        }
    }

    /// Return the critical threshold in % * 100.
    #[must_use]
    pub const fn crit_thresh(&self) -> u16 {
        self.crit_thresh
    }

    /// Return the limited threshold in % * 100.
    #[must_use]
    pub const fn limit_thresh(&self) -> u16 {
        self.limit_thresh
    }

    /// Return the suspended limit (LBT) in % * 100.
    #[must_use]
    pub const fn susp_limit(&self) -> u16 {
        self.susp_limit
    }
}
