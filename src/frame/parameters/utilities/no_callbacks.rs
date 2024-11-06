//! No callbacks response frame.

use le_stream::derive::FromLeStream;

use crate::frame::Parameter;

const ID: u16 = 0x0007;

/// Indicates that there are currently no pending callbacks.
///
/// This frame is a response to the callback command.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response;

impl Parameter for Response {
    const ID: u16 = ID;
}
