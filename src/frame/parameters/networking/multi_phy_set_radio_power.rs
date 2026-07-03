//! Parameters for the [`Networking::multi_phy_set_radio_power`](crate::Networking::multi_phy_set_radio_power) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;

crate::frame::parameters::frame!(
    0x00FA,
    { phy_index: u8, power: i8 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(phy_index: u8, power: i8) -> Self {
                Self { phy_index, power }
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
