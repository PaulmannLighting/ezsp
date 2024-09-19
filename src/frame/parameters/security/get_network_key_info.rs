use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;
use le_stream::derive::{FromLeBytes, ToLeBytes};
use siliconlabs::zigbee::security::ManNetworkKeyInfo;
use siliconlabs::Status;

const ID: u16 = 0x0116;

#[derive(Clone, Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u32,
    network_key_info: ManNetworkKeyInfo,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = ManNetworkKeyInfo;

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|_| self.network_key_info)
    }
}
