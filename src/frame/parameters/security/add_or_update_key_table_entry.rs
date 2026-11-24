//! Parameters for the legacy EZSP key table write command.

use crate::Error;
use crate::ember::{Eui64, Status, key};

const IS_LINK_KEY: u8 = 1;

crate::frame::parameters::frame!(
    0x0066,
    { address: Eui64, link_key: u8, key: key::Data },
    impl {
        impl Command {
            /// Insert or update a persistent link key for the partner.
            #[must_use]
            pub const fn link_key(address: Eui64, key: key::Data) -> Self {
                Self { address, link_key: IS_LINK_KEY, key }
            }
        }
    },
    { status: u8 } => Security(security)::AddOrUpdateKeyTableEntry,
    impl {
        impl TryFrom<Response> for () {
            type Error = Error;

            fn try_from(response: Response) -> Result<Self, Self::Error> {
                Status::check(response.status)
            }
        }
    }
);
