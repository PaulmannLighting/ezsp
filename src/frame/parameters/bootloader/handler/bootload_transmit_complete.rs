use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::types::ByteSizedVec;

crate::frame::parameters::handler!(
    0x0093,
    { status: u8, message: ByteSizedVec<u8> },
    impl {
        impl Handler {
            /// Returns `true` if an ACK was received from the destination or `false` if no ACK was received.
            ///
            /// # Errors
            ///
            /// Returns an [`Error`] if the status is not [`Status::Success`] or [`Status::DeliveryFailed`].
            pub fn ack_received(&self) -> Result<bool, Error> {
                match Status::from_u8(self.status).ok_or(self.status) {
                    Ok(Status::Success) => Ok(true),
                    Ok(Status::DeliveryFailed) => Ok(false),
                    other => Err(other.into()),
                }
            }

            /// The message that was sent.
            #[must_use]
            pub fn message(&self) -> &[u8] {
                self.message.as_ref()
            }
        }
    }
);
