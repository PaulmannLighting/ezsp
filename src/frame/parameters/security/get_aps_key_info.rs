//! Retrieve metadata about an APS link key. Does not retrieve contents.

use crate::ember::Eui64;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;
use le_stream::derive::{FromLeStream, ToLeStream};
use siliconlabs::zigbee::security::{ManApsKeyMetadata, ManContext};
use siliconlabs::Status;

const ID: u16 = 0x010C;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    context_in: ManContext,
}

impl Command {
    #[must_use]
    pub const fn new(context_in: ManContext) -> Self {
        Self { context_in }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub(crate) struct Response {
    payload: KeyInfo,
    status: u32,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = KeyInfo;

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|_| self.payload)
    }
}

/// The retrieved key information.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct KeyInfo {
    eui: Eui64,
    key_data: ManApsKeyMetadata,
}

impl KeyInfo {
    /// Returns the EUI64.
    #[must_use]
    pub const fn eui(&self) -> Eui64 {
        self.eui
    }

    /// Returns the key data.
    #[must_use]
    pub const fn key_data(&self) -> &ManApsKeyMetadata {
        &self.key_data
    }
}
