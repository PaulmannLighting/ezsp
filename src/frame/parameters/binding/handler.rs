//! Handlers for the binding commands.

pub use self::remote_delete_binding::Handler as RemoteDeleteBinding;
pub use self::remote_set_binding::Handler as RemoteSetBinding;

mod remote_delete_binding;
mod remote_set_binding;

/// The handler for the binding commands.
#[expect(variant_size_differences)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// The handler for the remote delete binding command.
    RemoteDeleteBinding(RemoteDeleteBinding),
    /// The handler for the remote set binding command.
    RemoteSetBinding(Box<RemoteSetBinding>),
}
