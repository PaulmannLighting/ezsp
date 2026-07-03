use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;

crate::frame::parameters::handler!(
    0x00C7,
    { status: u8, gpep_handle: u8 },
    impl {
        impl TryFrom<Handler> for u8 {
            type Error = Error;

            fn try_from(handler: Handler) -> Result<Self, Self::Error> {
                match Status::from_u8(handler.status).ok_or(handler.status) {
                    Ok(Status::Success) => Ok(handler.gpep_handle),
                    other => Err(other.into()),
                }
            }
        }
    }
);
