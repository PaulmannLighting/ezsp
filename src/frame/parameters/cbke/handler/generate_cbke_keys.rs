use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{PublicKeyData, Status};

crate::frame::parameters::handler!(
    0x009E,
    { status: u8, ephemeral_public_key: PublicKeyData },
    impl {
        /// Converts the handler into a [`PublicKeyData`] or an appropriate [`Error`]
        /// by evaluating its status field.
        impl TryFrom<Handler> for PublicKeyData {
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
