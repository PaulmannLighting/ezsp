//! Parameters for the [`Networking::get_next_beacon`](crate::Networking::get_next_beacon) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::beacon::Data;

crate::frame::parameters::frame!(
    0x0004,
    {},
    { status: u8, beacon: Data } => Networking(networking)::GetNextBeacon,
    impl {
        /// Convert a response into a [`Data`] or an appropriate [`Error`] depending on its status.
        impl TryFrom<Response> for Data {
            type Error = Error;

            fn try_from(response: Response) -> Result<Self, Self::Error> {
                match Status::from_u8(response.status).ok_or(response.status) {
                    Ok(Status::Success) => Ok(response.beacon),
                    other => Err(other.into()),
                }
            }
        }
    }
);
