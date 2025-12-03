//! Parameters for the [`Zll::set_data_token`](crate::Zll::set_data_token) command.

use le_stream::{FromLeStream, ToLeStream};

use crate::ember::zll::DataToken;
use crate::frame::Parameter;

const ID: u16 = 0x00BD;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    data: DataToken,
}

impl Command {
    #[must_use]
    pub const fn new(data: DataToken) -> Self {
        Self { data }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response;

impl Parameter for Response {
    const ID: u16 = ID;
}
