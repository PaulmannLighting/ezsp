//! Indicates that the NCP received an invalid command.

use core::fmt::Display;
use std::io::{Error, ErrorKind};

use num_traits::FromPrimitive;

use crate::ezsp::Status;

crate::frame::parameters::response!(
    0x0058,
    { reason: u8 },
    impl {
        impl Response {
            /// The unique ID of the [`Response`] frame type.
            pub const ID: u16 = <Self as crate::frame::Parameter>::ID;

            /// Returns the reason for the invalid command response.
            ///
            /// # Errors
            ///
            /// Returns an error if the reason is not a valid [`Status`].
            pub fn reason(&self) -> Result<Status, u8> {
                Status::from_u8(self.reason).ok_or(self.reason)
            }
        }

        impl Display for Response {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self.reason() {
                    Ok(status) => write!(f, "{status} ({status:#04X})"),
                    Err(reason) => write!(f, "Unknown reason: {reason:#04X}"),
                }
            }
        }

        impl From<Response> for Error {
            fn from(response: Response) -> Self {
                Self::new(
                    ErrorKind::Unsupported,
                    format!("Invalid command: {response}"),
                )
            }
        }
    }
);
