//! Parameters for the [`Wwah::is_hub_connected`](crate::Wwah::is_hub_connected) command.

crate::frame::parameters::frame!(
    0x00E6,
    {},
    { is_hub_connected: bool },
    impl {
        /// Convert the response into a boolean indicating if the hub is connected.
        impl From<Response> for bool {
            fn from(response: Response) -> Self {
                response.is_hub_connected
            }
        }
    }
);
