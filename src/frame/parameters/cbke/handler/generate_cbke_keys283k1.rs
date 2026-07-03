use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{PublicKey283k1Data, Status};

crate::frame::parameters::handler!(
    0x00E9,
    { status: u8, ephemeral_public_key: PublicKey283k1Data },
    impl {
        /// Converts the handler into a [`PublicKey283k1Data`] or an appropriate [`Error`]
        /// by evaluating its status field.
        impl TryFrom<Handler> for PublicKey283k1Data {
            type Error = Error;

            fn try_from(handler: Handler) -> Result<Self, Self::Error> {
                match Status::from_u8(handler.status).ok_or(handler.status) {
                    Ok(Status::Success) => Ok(handler.ephemeral_public_key),
                    other => Err(other.into()),
                }
            }
        }
    }
);
