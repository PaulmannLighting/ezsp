use std::convert::Infallible;

use super::{Callback, Response};

crate::frame::parameters::parameter_group_enum!(
    Parameters,
    Response,
    Callback,
    impl {
        /// Implementation to satisfy trait bound on `Into<Parameters>`.
        impl From<Infallible> for Parameters {
            fn from(value: Infallible) -> Self {
                match value {}
            }
        }
    },
);
