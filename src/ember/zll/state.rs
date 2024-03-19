use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum State {
    None = 0x0000,
    FactoryNew = 0x0001,
    AddressAssignmentCapable = 0x0002,
    LinkInitiator = 0x0010,
    LinkPriorityRequest = 0x0020,
    NonZllNetwork = 0x0100,
}

impl From<State> for u16 {
    fn from(state: State) -> Self {
        state
            .to_u16()
            .expect("State should always be convertible to u16.")
    }
}

impl TryFrom<u16> for State {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or(value)
    }
}
