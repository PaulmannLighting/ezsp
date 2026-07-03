use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::types::ByteSizedVec;

crate::frame::parameters::handler!(
    0x00A7,
    { status: u8, message: ByteSizedVec<u8> },
    impl {
        /// Converts the handler into an array of bytes or an
        /// appropriate [`Error`] by evaluating its status field.
        impl TryFrom<Handler> for ByteSizedVec<u8> {
            type Error = Error;

            fn try_from(handler: Handler) -> Result<Self, Self::Error> {
                match Status::from_u8(handler.status).ok_or(handler.status) {
                    Ok(Status::Success) => Ok(handler.message),
                    other => Err(other.into()),
                }
            }
        }
    }
);
