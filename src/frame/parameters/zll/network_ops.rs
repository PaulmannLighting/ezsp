//! Parameters for the [`Zll::network_ops`](crate::Zll::network_ops) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::zll::Network;
use crate::ezsp::zll::NetworkOperation;

crate::frame::parameters::frame!(0x00B2, { network_info: Network, op: u8, radio_tx_power: i8 }, { status: u8 });

impl Command {
    #[must_use]
    pub fn new(network_info: Network, op: NetworkOperation, radio_tx_power: i8) -> Self {
        Self {
            network_info,
            op: op.into(),
            radio_tx_power,
        }
    }
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
