use crate::ember::Status;
use crate::ezsp::network::InitBitmask;
use le_stream::derive::{FromLeBytes, ToLeBytes};
use num_traits::ToPrimitive;

pub const ID: u16 = 0x0017;

type Bitmask = heapless::Vec<InitBitmask, 2>;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    bitmask: u16,
}

impl Command {
    #[must_use]
    pub const fn new(bitmask: Bitmask) -> Self {
        Self {
            bitmask: bitmask.iter().filter_map(|bitmask| bitmask.to_u16()).sum(),
        }
    }

    #[must_use]
    pub const fn network_init_struct(&self) -> Bitmask {
        let mut bitmask = Bitmask::new();

        if self.bitmask & InitBitmask::ParentInfoInToken.into() {
            bitmask.push(InitBitmask::ParentInfoInToken)
        }

        if self.bitmask & InitBitmask::EndDeviceRejoinOnReboot.into() {
            bitmask.push(InitBitmask::EndDeviceRejoinOnReboot)
        }

        bitmask
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
