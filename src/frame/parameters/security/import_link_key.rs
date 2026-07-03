//! Parameters for the [`Security::import_link_key`](crate::Security::import_link_key) command.

use num_traits::FromPrimitive;
use silizium::Status;
use silizium::zigbee::security::man::Key;

use crate::Error;
use crate::ember::Eui64;

crate::frame::parameters::frame!(
    0x010E,
    { index: u8, address: Eui64, plaintext_key: Key },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(index: u8, address: Eui64, plaintext_key: Key) -> Self {
                Self {
                    index,
                    address,
                    plaintext_key,
                }
            }
        }
    },
    { status: u32 },
    impl {
        /// Convert the response into `()` or an appropriate [`Error`] depending on its status.
        impl TryFrom<Response> for () {
            type Error = Error;

            fn try_from(response: Response) -> Result<Self, Self::Error> {
                match Status::from_u32(response.status).ok_or(response.status) {
                    Ok(Status::Ok) => Ok(()),
                    other => Err(other.into()),
                }
            }
        }
    }
);
