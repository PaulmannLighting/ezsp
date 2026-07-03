//! Handlers for the binding commands.

pub use self::remote_delete_binding::Handler as RemoteDeleteBinding;
pub use self::remote_set_binding::Handler as RemoteSetBinding;

mod remote_delete_binding;
mod remote_set_binding;
crate::frame::parameters::parameter_enum!(Handler, RemoteDeleteBinding, RemoteSetBinding);
