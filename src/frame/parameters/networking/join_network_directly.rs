//! Parameters for the [`Networking::join_network_directly`](crate::Networking::join_network_directly) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::beacon::Data;
use crate::ember::node::Type;

crate::frame::parameters::frame!(
    0x003B,
    { local_node_type: u8, beacon: Data, radio_tx_power: i8, clear_beacons_after_network_up: bool },
    impl {
        impl Command {
            /// Creates command parameters.
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
    },
    { status: u8 },
    impl {
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
    }
);
