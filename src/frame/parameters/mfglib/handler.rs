//! `Mfglib` event handler.

pub use self::rx::Handler as Rx;

mod rx;
crate::frame::parameters::parameter_enum!(Handler, Rx);
