//! Parameters for the [`Zll::rx_on_when_idle_get_active`](crate::Zll::rx_on_when_idle_get_active) command.

crate::frame::parameters::frame!(
    0x00D8,
    {},
    { zll_rx_on_when_idle_get_active: bool } => Zll(zll)::RxOnWhenIdleGetActive,
    impl {
        impl Response {
            /// ZLL radio on when idle mode is active?
            #[must_use]
            pub const fn zll_rx_on_when_idle_get_active(&self) -> bool {
                self.zll_rx_on_when_idle_get_active
            }
        }
    }
);
