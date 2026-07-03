use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;

crate::frame::parameters::handler!(
    0x0078,
    { status: u8 },
    impl {
        impl Handler {
            /// The result of the DSA verification operation.
            ///
            /// # Returns
            ///
            /// Returns `true` if the signature is valid, `false` if it failed.
            ///
            /// # Errors
            ///
            /// Returns an [`Error`] if the status is invalid.
            pub fn is_valid(&self) -> Result<bool, Error> {
                match Status::from_u8(self.status).ok_or(self.status) {
                    Ok(Status::Success) => Ok(true),
                    Ok(Status::SignatureVerifyFailure) => Ok(false),
                    other => Err(other.into()),
                }
            }
        }
    }
);
