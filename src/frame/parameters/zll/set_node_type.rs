//! Parameters for the [`Zll::set_node_type`](crate::Zll::set_node_type) command.

use crate::ember::node::Type;

crate::frame::parameters::frame!(
    0x00D5,
    { node_type: u8 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub fn new(node_type: Type) -> Self {
                Self {
                    node_type: node_type.into(),
                }
            }
        }
    },
    {}
);
