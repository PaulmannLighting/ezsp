//! Parameters for the [`Wwah::get_parent_classification_enabled`](crate::Wwah::get_parent_classification_enabled) command.

crate::frame::parameters::frame!(
    0x00F0,
    {},
    { enabled: bool } => Wwah(wwah)::GetParentClassificationEnabled,
    impl {
        /// Convert the response into a boolean indicating if the parent classification is enabled.
        impl From<Response> for bool {
            fn from(response: Response) -> Self {
                response.enabled
            }
        }
    }
);
