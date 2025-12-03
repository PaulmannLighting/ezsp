//! Parameters for the [`Zll::start_scan`](crate::Zll::start_scan) command.

use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::node::Type;
use crate::frame::Parameter;

const ID: u16 = 0x00B4;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    channel_mask: u32,
    radio_power_for_scan: i8,
    node_type: u8,
}

impl Command {
    /// Create a new command to start a scan..
    #[must_use]
    pub fn new(channel_mask: u32, radio_power_for_scan: i8, node_type: Type) -> Self {
        Self {
            channel_mask,
            radio_power_for_scan,
            node_type: node_type.into(),
        }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Convert the response into a [`Result<()>`](crate::Result) by evaluating its status field.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
