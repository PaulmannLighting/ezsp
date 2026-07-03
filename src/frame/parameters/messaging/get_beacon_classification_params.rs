//! Parameters for the [`Messaging::get_beacon_classification_params`](crate::Messaging::get_beacon_classification_params) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::beacon::ClassificationParams;

crate::frame::parameters::frame!(
    0x00F3,
    {},
    { status: u8, param: ClassificationParams } => Messaging(messaging)::GetBeaconClassificationParams,
    impl {
        /// Converts the response into the [`ClassificationParams`] or an appropriate [`Error`] depending on its status.
        impl TryFrom<Response> for ClassificationParams {
            type Error = Error;

            fn try_from(response: Response) -> Result<Self, Self::Error> {
                match Status::from_u8(response.status).ok_or(response.status) {
                    Ok(Status::Success) => Ok(response.param),
                    other => Err(other.into()),
                }
            }
        }
    }
);
