//! Parameters for the [`Zll::set_data_token`](crate::Zll::set_data_token) command.

use crate::ember::zll::DataToken;

crate::frame::parameters::frame!(
    0x00BD,
    { data: DataToken },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(data: DataToken) -> Self {
                Self { data }
            }
        }
    },
    {}
);
