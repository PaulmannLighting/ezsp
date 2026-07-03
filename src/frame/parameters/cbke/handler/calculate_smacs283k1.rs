use num_traits::FromPrimitive;

use super::Payload;
use crate::Error;
use crate::ember::Status;

crate::frame::parameters::handler!(
    0x00EB,
    { status: u8, payload: Payload },
    impl {
        /// Converts the handler into a [`Payload`] or an appropriate [`Error`] by evaluating its status field.
        impl TryFrom<Handler> for Payload {
            type Error = Error;

            fn try_from(handler: Handler) -> Result<Self, Self::Error> {
                match Status::from_u8(handler.status).ok_or(handler.status) {
                    Ok(Status::Success) => Ok(handler.payload),
                    other => Err(other.into()),
                }
            }
        }
    }
);
