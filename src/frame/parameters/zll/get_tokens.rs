use crate::ember::zll::{DataToken, SecurityToken};
use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00BC;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    data: DataToken,
    security: SecurityToken,
}

impl Response {
    #[must_use]
    pub const fn data(&self) -> &DataToken {
        &self.data
    }

    #[must_use]
    pub const fn security(&self) -> &SecurityToken {
        &self.security
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
