//! Parameters for the [`Cbke::save_preinstalled_cbke_data283k1`](crate::Cbke::save_preinstalled_cbke_data283k1) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;

crate::frame::parameters::frame!(
    0x00ED,
    {},
    { status: u8 },
    impl {
        /// Converts the response into `()` or an appropriate [`Error`] by evaluating its status field.
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
