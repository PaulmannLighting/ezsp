use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;
use le_stream::derive::{FromLeStream, ToLeStream};
use siliconlabs::zigbee::security::ManNetworkKeyInfo;
use siliconlabs::Status;

const ID: u16 = 0x0116;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter<u16> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u32,
    network_key_info: ManNetworkKeyInfo,
}

impl Parameter<u16> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = ManNetworkKeyInfo;

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|_| self.network_key_info)
    }
}
