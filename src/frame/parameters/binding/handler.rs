pub mod remote_delete_binding;
pub mod remote_set_binding;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    RemoteDeleteBinding(remote_delete_binding::Handler),
    RemoteSetBinding(remote_set_binding::Handler),
}

impl From<remote_delete_binding::Handler> for Handler {
    fn from(handler: remote_delete_binding::Handler) -> Self {
        Self::RemoteDeleteBinding(handler)
    }
}

impl From<remote_set_binding::Handler> for Handler {
    fn from(handler: remote_set_binding::Handler) -> Self {
        Self::RemoteSetBinding(handler)
    }
}
