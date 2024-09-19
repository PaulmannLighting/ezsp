use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::{DeviceDutyCycles, Status};
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x004C;

#[derive(Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    max_devices: u8,
}

impl Command {
    #[must_use]
    pub const fn new(max_devices: u8) -> Self {
        Self { max_devices }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    device_duty_cycles: DeviceDutyCycles,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = DeviceDutyCycles;

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|()| self.device_duty_cycles)
    }
}
