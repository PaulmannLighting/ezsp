//! Parameters for the [`Zll::operation_in_progress`](crate::Zll::operation_in_progress) command.

crate::frame::parameters::frame!(
    0x00D7,
    {},
    { zll_operation_in_progress: bool },
    impl {
        impl Response {
            /// Returns whether a ZLL operation is in progress.
            #[must_use]
            pub const fn zll_operation_in_progress(&self) -> bool {
                self.zll_operation_in_progress
            }
        }
    }
);
