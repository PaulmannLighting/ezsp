use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;

crate::frame::parameters::handler!(
    0x001C,
    { channel: u8, status: u8 },
    impl {
        impl Handler {
            /// The channel on which the current error occurred.
            ///
            /// Undefined for the case of [`Status::Success`].
            #[must_use]
            pub fn channel(&self) -> Option<u8> {
                match self.status() {
                    Ok(()) => None,
                    Err(_) => Some(self.channel),
                }
            }

            /// The error condition that occurred on the current channel.
            ///
            /// Value will be [`Status::Success`] when the scan has completed.
            ///
            /// # Errors
            ///
            /// Returns an [`Error`] if the status is invalid.
            pub fn status(&self) -> Result<(), Error> {
                match Status::from_u8(self.status).ok_or(self.status) {
                    Ok(Status::Success) => Ok(()),
                    other => Err(other.into()),
                }
            }
        }

        impl TryFrom<Handler> for u8 {
            type Error = Error;

            fn try_from(handler: Handler) -> Result<Self, Self::Error> {
                handler.status().map(|()| handler.channel)
            }
        }
    }
);
