use crate::frame::Identified;
use le_stream::derive::FromLeStream;

const ID: u16 = 0x0007;

/// Indicates that there are currently no pending callbacks.
///
/// This frame is a response to the callback command.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response;

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
