//! Parameters for the [`Networking::find_and_rejoin_network`](crate::Networking::find_and_rejoin_network) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;

crate::frame::parameters::frame!(
    0x0021,
    { have_current_network_key: bool, channel_mask: u32 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(have_current_network_key: bool, channel_mask: u32) -> Self {
                Self {
                    have_current_network_key,
                    channel_mask,
                }
            }
        }
    },
    { status: u8 } => Networking(networking)::FindAndRejoinNetwork,
    impl {
        /// Converts the response into `()` or an appropriate [`Error`] depending on its status.
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
