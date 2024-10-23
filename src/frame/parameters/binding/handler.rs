//! Handlers for the binding commands.

use le_stream::FromLeStream;

use crate::error::Decode;
use crate::frame::Parameter;

use crate::ashv2::Parsable;
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

impl Parsable for Handler {
    fn parse_from_le_stream<T>(id: u16, stream: T) -> Result<Self, Decode>
    where
        T: Iterator<Item = u8>,
    {
        match id {
            <RemoteDeleteBinding as Parameter>::ID => Ok(Self::RemoteDeleteBinding(
                RemoteDeleteBinding::from_le_stream_exact(stream)?,
            )),
            <RemoteSetBinding as Parameter>::ID => Ok(Self::RemoteSetBinding(
                RemoteSetBinding::from_le_stream_exact(stream)?,
            )),
            _ => Err(Decode::InvalidFrameId(id)),
        }
    }
}
