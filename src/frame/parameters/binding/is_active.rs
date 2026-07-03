//! Parameters for the [`Binding::binding_is_active`](crate::Binding::is_active) command.

crate::frame::parameters::frame!(
    0x002E,
    { index: u8 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(index: u8) -> Self {
                Self { index }
            }
        }
    },
    { active: bool } => Binding(binding)::IsActive,
    impl {
        impl Response {
            /// True if the binding table entry is active, false otherwise.
            #[must_use]
            pub const fn active(&self) -> bool {
                self.active
            }
        }
    }
);
