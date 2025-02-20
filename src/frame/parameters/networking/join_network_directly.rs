//! Parameters for the [`Networking::join_network_directly`](crate::Networking::join_network_directly) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::beacon::Data;
use crate::ember::node::Type;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x003B;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    local_node_type: u8,
    beacon: Data,
    radio_tx_power: i8,
    clear_beacons_after_network_up: bool,
}

impl Command {
    #[must_use]
    pub fn new(
        local_node_type: Type,
        beacon: Data,
        radio_tx_power: i8,
        clear_beacons_after_network_up: bool,
    ) -> Self {
        Self {
            local_node_type: local_node_type.into(),
            beacon,
            radio_tx_power,
            clear_beacons_after_network_up,
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

/// Convert a response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
