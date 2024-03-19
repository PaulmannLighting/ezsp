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
    pub fn new(bitmask: Bitmask) -> Self {
        Self {
            bitmask: bitmask.iter().filter_map(ToPrimitive::to_u16).sum(),
        }
    }

    #[must_use]
    pub fn network_init_struct(&self) -> Bitmask {
        let mut bitmask = Bitmask::new();

        if self.bitmask & Into::<u16>::into(InitBitmask::ParentInfoInToken) != 0 {
            bitmask
                .push(InitBitmask::ParentInfoInToken)
                .expect("Bitmask buffer should have sufficient capacity.");
        }

        if self.bitmask & Into::<u16>::into(InitBitmask::EndDeviceRejoinOnReboot) != 0 {
            bitmask
                .push(InitBitmask::EndDeviceRejoinOnReboot)
                .expect("Bitmask buffer should have sufficient capacity.");
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
