use crate::ember::node::Type;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::resolve::Resolve;
use crate::Error;
use le_stream::derive::{FromLeStream, ToLeStream};

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
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub(crate) struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
