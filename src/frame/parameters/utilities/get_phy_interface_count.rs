//! Parameters for the [`Utilities::get_phy_interface_count`](crate::Utilities::get_phy_interface_count) command.

crate::frame::parameters::frame!(
    0x00FC,
    {},
    { interface_count: u8 },
    impl {
        /// Convert the response into the number of physical interfaces.
        impl From<Response> for u8 {
            fn from(response: Response) -> Self {
                response.interface_count
            }
        }
    }
);
