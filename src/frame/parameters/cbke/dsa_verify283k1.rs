//! Parameters for the [`Cbke::dsa_verify`](crate::Cbke::dsa_verify) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{Certificate283k1Data, MessageDigest, Signature283k1Data, Status};

crate::frame::parameters::frame!(
    0x00B0,
    { digest: MessageDigest, signer_certificate: Certificate283k1Data, received_sig: Signature283k1Data },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(
                digest: MessageDigest,
                signer_certificate: Certificate283k1Data,
                received_sig: Signature283k1Data,
            ) -> Self {
                Self {
                    digest,
                    signer_certificate,
                    received_sig,
                }
            }
        }
    },
    { status: u8 },
    impl {
        /// Converts the response into `()` or an appropriate [`Error`] by evaluating its status field.
        impl TryFrom<Response> for () {
            type Error = Error;

            fn try_from(response: Response) -> Result<Self, Self::Error> {
                match Status::from_u8(response.status).ok_or(response.status) {
                    Ok(Status::Success) => Ok(()),
                    other => Err(other.into()),
                }
            }
        }
    }
);
