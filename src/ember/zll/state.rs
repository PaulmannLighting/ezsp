use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Ember ZLL state.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u16)]
pub enum State {
    /// No state.
    None = 0x0000,
    /// The device is factory new.
    FactoryNew = 0x0001,
    /// The device is capable of assigning addresses to other devices.
    AddressAssignmentCapable = 0x0002,
    /// The device is initiating a link operation.
    LinkInitiator = 0x0010,
    /// The device is requesting link priority.
    LinkPriorityRequest = 0x0020,
    /// The device is on a non-ZLL network.
    NonZllNetwork = 0x0100,
}

impl From<State> for u16 {
    fn from(state: State) -> Self {
        state as Self
    }
}

impl TryFrom<u16> for State {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or(value)
    }
}
