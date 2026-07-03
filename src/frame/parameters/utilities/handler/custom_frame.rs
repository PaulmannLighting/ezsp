//! Custom frame handler.

use crate::types::ByteSizedVec;

crate::frame::parameters::handler!(
    0x0054,
    { payload: ByteSizedVec<u8> },
    impl {
        impl Handler {
            /// The payload of the custom frame.
            #[must_use]
            pub fn payload(&self) -> &[u8] {
                self.payload.as_ref()
            }
        }
    }
);
