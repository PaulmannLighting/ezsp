use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::error::Resolve;
use crate::Error;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Status {
    NoNetwork = 0x00,
    JoiningNetwork = 0x01,
    JoinedNetwork = 0x02,
    JoinedNetworkNoParent = 0x03,
    LeavingNetwork = 0x04,
}

impl From<Status> for u8 {
    fn from(status: Status) -> Self {
        status
            .to_u8()
            .expect("Status should always be convertible to u8.")
    }
}

impl TryFrom<u8> for Status {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

impl Resolve for Result<Status, u8> {
    type Result = Status;

    fn resolve(self) -> Result<Self::Result, Error> {
        self.map_err(|status| Error::InvalidEmberNetworkStatus(status))
    }
}
