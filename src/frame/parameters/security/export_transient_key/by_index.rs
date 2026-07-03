//! Parameters for the [`Security::export_transient_key_by_index`](crate::Security::export_transient_key_by_index) command.

use num_traits::FromPrimitive;
use silizium::Status;

use super::transient_key::TransientKey;
use crate::Error;

crate::frame::parameters::frame!(
    0x0112,
    { index: u8 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(index: u8) -> Self {
                Self { index }
            }
        }
    },
    { payload: TransientKey, status: u32 },
    impl {
        /// Convert the response into [`TransientKey`] or an appropriate [`Error`] depending on its status.
        impl TryFrom<Response> for TransientKey {
            type Error = Error;

            fn try_from(response: Response) -> Result<Self, Self::Error> {
                match Status::from_u32(response.status).ok_or(response.status) {
                    Ok(Status::Ok) => Ok(response.payload),
                    other => Err(other.into()),
                }
            }
        }
    }
);
