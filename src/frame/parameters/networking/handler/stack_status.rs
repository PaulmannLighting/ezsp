use num_traits::FromPrimitive;

use crate::ember::Status;

crate::frame::parameters::handler!(
    0x0019,
    { status: u8 },
    impl {
        impl Handler {
            /// Stack status.
            ///
            /// # Errors
            ///
            /// Returns an error if the status is invalid.
            pub fn result(&self) -> Result<Status, u8> {
                Status::from_u8(self.status).ok_or(self.status)
            }
        }
    }
);
