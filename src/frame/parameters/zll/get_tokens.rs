//! Parameters for the [`Zll::get_tokens`](crate::Zll::get_tokens) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::zll::{DataToken, SecurityToken};
use crate::frame::Parameter;

const ID: u16 = 0x00BC;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    data: DataToken,
    security: SecurityToken,
}

impl Response {
    /// Returns the token data.
    #[must_use]
    pub const fn data(&self) -> &DataToken {
        &self.data
    }

    /// Returns the token security.
    #[must_use]
    pub const fn security(&self) -> &SecurityToken {
        &self.security
    }
}

impl Parameter for Response {
    const ID: u16 = ID;
}
