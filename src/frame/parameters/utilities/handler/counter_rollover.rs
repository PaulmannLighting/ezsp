//! Counter Rollover Handler

use crate::ember::counter::Type;

crate::frame::parameters::handler!(
    0x00F2,
    { typ: u8 },
    impl {
        impl Handler {
            /// Type of Counter.
            ///
            /// # Errors
            ///
            /// Returns an error if the counter type is invalid.
            pub fn typ(&self) -> Result<Type, u8> {
                Type::try_from(self.typ)
            }
        }
    }
);
