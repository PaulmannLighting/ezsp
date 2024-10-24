use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::node::Type;
use crate::ember::Status;
use crate::frame::Identified;
use crate::Error;

const ID: u16 = 0x00B4;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
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

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
