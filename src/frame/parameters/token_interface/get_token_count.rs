//! Parameters for the [`TokenInterface::get_token_count`](crate::TokenInterface::get_token_count) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x0100;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    count: u8,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Convert the response into the count of tokens.
impl From<Response> for u8 {
    fn from(response: Response) -> Self {
        response.count
    }
}
