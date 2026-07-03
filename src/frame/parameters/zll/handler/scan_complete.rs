use num_traits::FromPrimitive;

use crate::ember::Status;

crate::frame::parameters::handler!(
    0x00B7,
    { status: u8 },
    impl {
        impl Handler {
            /// Status of the operation.
            ///
            /// # Errors
            ///
            /// Returns an error if the status is not success.
            pub fn result(&self) -> Result<Status, u8> {
                Status::from_u8(self.status).ok_or(self.status)
            }
        }
    }
);
