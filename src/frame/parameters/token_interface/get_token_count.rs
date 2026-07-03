//! Parameters for the [`TokenInterface::get_token_count`](crate::TokenInterface::get_token_count) command.

crate::frame::parameters::frame!(
    0x0100,
    {},
    { count: u8 } => TokenInterface(token_interface)::GetTokenCount,
    impl {
        /// Convert the response into the count of tokens.
        impl From<Response> for u8 {
            fn from(response: Response) -> Self {
                response.count
            }
        }
    }
);
