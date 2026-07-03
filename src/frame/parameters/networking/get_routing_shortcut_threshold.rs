//! Parameters for the [`Networking::get_routing_shortcut_threshold`](crate::Networking::get_routing_shortcut_threshold) command.

crate::frame::parameters::frame!(
    0x00D1,
    {},
    { routing_shortcut_thresh: u8 } => Networking(networking)::GetRoutingShortcutThreshold,
    impl {
        impl Response {
            /// Returns the routing shortcut threshold.
            #[must_use]
            pub const fn routing_shortcut_thresh(&self) -> u8 {
                self.routing_shortcut_thresh
            }
        }
    }
);
