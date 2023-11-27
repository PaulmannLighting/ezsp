use le_stream::derive::{FromLeBytes, ToLeBytes};
use siliconlabs::zigbee::security::ManNetworkKeyInfo;
use siliconlabs::Status;

pub const ID: u16 = 0x0116;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u32,
    network_key_info: ManNetworkKeyInfo,
}

impl Response {
    #[must_use]
    pub fn new(status: Status, network_key_info: ManNetworkKeyInfo) -> Self {
        Self {
            status: status.into(),
            network_key_info,
        }
    }

    pub fn status(&self) -> Result<Status, u32> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn network_key_info(&self) -> &ManNetworkKeyInfo {
        &self.network_key_info
    }
}
