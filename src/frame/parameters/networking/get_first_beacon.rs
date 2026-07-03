//! Parameters for the [`Networking::get_first_beacon`](crate::Networking::get_first_beacon) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::beacon::Iterator;

crate::frame::parameters::frame!(0x003D, {}, { status: u8, beacon_iterator: Iterator });

/// Converts the response into [`Iterator`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for Iterator {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.beacon_iterator),
            other => Err(other.into()),
        }
    }
}
