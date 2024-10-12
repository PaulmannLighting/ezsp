use crate::frame::Parameter;
use le_stream::derive::FromLeStream;

const ID: u16 = 0x0007;

/// The response to this command can be any of the callback responses.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response;

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
