//! Parameters for the [`Networking::multi_phy_start`](crate::Networking::multi_phy_start) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::multi_phy::nwk::Config;

crate::frame::parameters::frame!(
    0x00F8,
    { phy_index: u8, page: u8, channel: u8, power: i8, bitmask: u8 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub fn new(phy_index: u8, page: u8, channel: u8, power: i8, bitmask: Config) -> Self {
                Self {
                    phy_index,
                    page,
                    channel,
                    power,
                    bitmask: bitmask.into(),
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
