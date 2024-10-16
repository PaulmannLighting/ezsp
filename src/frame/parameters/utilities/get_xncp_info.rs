use crate::ember::Status;
use crate::frame::Parameter;
use crate::resolve::Resolve;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0013;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    payload: Option<Payload>,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = Payload;

    fn resolve(self) -> crate::Result<Self::Output> {
        Status::try_from(self.status).resolve().and_then(|_| {
            self.payload
                .ok_or(crate::Error::Custom("No payload.".into()))
        })
    }
}

/// Payload of the get XNCP info command.
#[derive(Clone, Copy, Debug, Eq, PartialEq, FromLeStream)]
pub struct Payload {
    manufacturer_id: u16,
    version_number: u16,
}

impl Payload {
    /// Returns the manufacturer ID.
    #[must_use]
    pub const fn manufacturer_id(self) -> u16 {
        self.manufacturer_id
    }

    /// Returns the version number.
    #[must_use]
    pub const fn version_number(self) -> u16 {
        self.version_number
    }
}
