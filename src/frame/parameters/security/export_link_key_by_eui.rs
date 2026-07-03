//! Parameters for the [`Security::export_link_key_by_eui`](crate::Security::export_link_key_by_eui) command.

use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use silizium::Status;
use silizium::zigbee::security::man::{ApsKeyMetadata, Key};

use crate::Error;
use crate::ember::Eui64;

crate::frame::parameters::frame!(
    0x010D,
    { eui: Eui64 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(eui: Eui64) -> Self {
                Self { eui }
            }
        }
    },
    { payload: Payload, status: u32 },
    impl {
        /// Convert the response into [`Payload`] or an appropriate [`Error`] depending on its status.
        impl TryFrom<Response> for Payload {
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

/// Payload of the export link key by EUI64 command.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Payload {
    plaintext_key: Key,
    index: u8,
    key_data: ApsKeyMetadata,
}

impl Payload {
    /// Returns the plain text key.
    #[must_use]
    pub const fn plaintext_key(&self) -> &Key {
        &self.plaintext_key
    }

    /// Returns the index.
    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }

    /// Returns the key metadata.
    #[must_use]
    pub const fn key_data(&self) -> &ApsKeyMetadata {
        &self.key_data
    }
}
