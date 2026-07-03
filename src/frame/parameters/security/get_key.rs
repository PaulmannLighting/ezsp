//! Parameters for the [`Security::get_key`](crate::Security::get_key) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::key::{Struct, Type};

crate::frame::parameters::frame!(
    0x006A,
    { key: u8 },
    impl {
        impl Command {
            /// Creates a new `Command`.
            #[must_use]
            pub const fn new(key: Type) -> Self {
                Self { key: key as u8 }
            }
        }
    },
    { status: u8, key: Struct },
    impl {
        impl TryFrom<Response> for Struct {
            type Error = Error;

            fn try_from(response: Response) -> Result<Self, Self::Error> {
                match Status::from_u8(response.status).ok_or(response.status) {
                    Ok(Status::Success) => Ok(response.key),
                    other => Err(other.into()),
                }
            }
        }
    }
);
