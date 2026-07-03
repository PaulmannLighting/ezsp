//! Parameters for the [`Networking::set_logical_and_radio_channel`](crate::Networking::set_logical_and_radio_channel) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;

crate::frame::parameters::frame!(
    0x00B9,
    { radio_channel: u8 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(radio_channel: u8) -> Self {
                Self { radio_channel }
            }
        }
    },
    { status: u8 } => Networking(networking)::SetLogicalAndRadioChannel,
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
