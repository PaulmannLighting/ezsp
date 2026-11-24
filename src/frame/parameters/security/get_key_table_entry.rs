//! Parameters for the legacy EZSP key table read command.

use crate::Error;
use crate::ember::{Status, key};

crate::frame::parameters::frame!(
    0x0071,
    { index: u8 },
    impl {
        impl Command {
            /// Select a legacy key table entry by index.
            #[must_use]
            pub const fn new(index: u8) -> Self {
                Self { index }
            }
        }
    },
    { status: u8, key: key::Struct } => Security(security)::GetKeyTableEntry,
    impl {
        impl TryFrom<Response> for Option<key::Struct> {
            type Error = Error;

            fn try_from(response: Response) -> Result<Self, Self::Error> {
                if response.status == u8::from(Status::TableEntryErased) {
                    return Ok(None);
                }
                Status::check(response.status)?;
                Ok(Some(response.key))
            }
        }
    }
);
