//! Handlers for the binding commands.

pub use remote_delete_binding::Handler as RemoteDeleteBinding;
pub use remote_set_binding::Handler as RemoteSetBinding;

mod remote_delete_binding;
mod remote_set_binding;

/// The handler for the binding commands.
#[allow(variant_size_differences, clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// The handler for the remote delete binding command.
    RemoteDeleteBinding(RemoteDeleteBinding),
    /// The handler for the remote set binding command.
    RemoteSetBinding(RemoteSetBinding),
}

impl From<RemoteDeleteBinding> for Handler {
    fn from(handler: RemoteDeleteBinding) -> Self {
        Self::RemoteDeleteBinding(handler)
    }
}

impl From<RemoteSetBinding> for Handler {
    fn from(handler: RemoteSetBinding) -> Self {
        Self::RemoteSetBinding(handler)
    }
}
