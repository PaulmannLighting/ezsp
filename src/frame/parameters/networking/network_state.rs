//! Parameters for the [`Networking::network_state`](crate::Networking::network_state) command.

use num_traits::FromPrimitive;

use crate::ember::network::Status;
use crate::{Error, ValueError};

crate::frame::parameters::frame!(
    0x0018,
    {},
    { status: u8 } => Networking(networking)::NetworkState,
    impl {
        /// Convert a response into a [`Status`] or an appropriate [`Error`] depending on its status.
        impl TryFrom<Response> for Status {
            type Error = Error;

            fn try_from(response: Response) -> Result<Self, Self::Error> {
                Self::from_u8(response.status)
                    .ok_or_else(|| ValueError::EmberNetworkStatus(response.status).into())
            }
        }
    }
);
