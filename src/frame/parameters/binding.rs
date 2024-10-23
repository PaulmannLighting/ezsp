//! Binding Frames

use le_stream::FromLeStream;

use crate::error::Decode;
use crate::frame::parse::Parse;
use crate::frame::Parameter;

pub(crate) mod binding_is_active;
pub(crate) mod clear_binding_table;
pub(crate) mod delete_binding;
pub(crate) mod get_binding;
pub(crate) mod get_binding_remote_node_id;
pub mod handler;
pub(crate) mod set_binding;
pub(crate) mod set_binding_remote_node_id;

/// Command parameters for binding frames.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(variant_size_differences)]
pub enum Command {
    /// Command parameters for [`Binding::binding_is_active()`](crate::Binding::binding_is_active).
    BindingIsActive(binding_is_active::Command),
    /// Command parameters for [`Binding::clear_binding_table()`](crate::Binding::clear_binding_table).
    ClearBindingTable(clear_binding_table::Command),
    /// Command parameters for [`Binding::delete_binding()`](crate::Binding::delete_binding).
    DeleteBinding(delete_binding::Command),
    /// Command parameters for [`Binding::get_binding()`](crate::Binding::get_binding).
    GetBinding(get_binding::Command),
    /// Command parameters for [`Binding::get_binding_remote_node_id()`](crate::Binding::get_binding_remote_node_id).
    GetBindingRemoteNodeId(get_binding_remote_node_id::Command),
    /// Command parameters for [`Binding::set_binding()`](crate::Binding::set_binding).
    SetBinding(set_binding::Command),
    /// Command parameters for [`Binding::set_binding_remote_node_id()`](crate::Binding::set_binding_remote_node_id).
    SetBindingRemoteNodeId(set_binding_remote_node_id::Command),
}

/// Response parameters for binding frames.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    /// Response parameters for [`Binding::binding_is_active()`](crate::Binding::binding_is_active).
    BindingIsActive(binding_is_active::Response),
    /// Response parameters for [`Binding::clear_binding_table()`](crate::Binding::clear_binding_table).
    ClearBindingTable(clear_binding_table::Response),
    /// Response parameters for [`Binding::delete_binding()`](crate::Binding::delete_binding).
    DeleteBinding(delete_binding::Response),
    /// Response parameters for [`Binding::get_binding()`](crate::Binding::get_binding).
    GetBinding(get_binding::Response),
    /// Response parameters for [`Binding::get_binding_remote_node_id()`](crate::Binding::get_binding_remote_node_id).
    GetBindingRemoteNodeId(get_binding_remote_node_id::Response),
    /// Response parameters for [`Binding::set_binding()`](crate::Binding::set_binding).
    SetBinding(set_binding::Response),
    /// Response parameters for [`Binding::set_binding_remote_node_id()`](crate::Binding::set_binding_remote_node_id).
    SetBindingRemoteNodeId(set_binding_remote_node_id::Response),
    /// Response parameters for callback handlers.
    Handler(handler::Handler),
}

impl Parse for Response {
    fn parse<T>(id: u16, stream: T) -> Result<Self, Decode>
    where
        T: Iterator<Item = u8>,
    {
        match id {
            <binding_is_active::Response as Parameter>::ID => Ok(Self::BindingIsActive(
                binding_is_active::Response::from_le_stream_exact(stream)?,
            )),
            <clear_binding_table::Response as Parameter>::ID => Ok(Self::ClearBindingTable(
                clear_binding_table::Response::from_le_stream_exact(stream)?,
            )),
            <delete_binding::Response as Parameter>::ID => Ok(Self::DeleteBinding(
                delete_binding::Response::from_le_stream_exact(stream)?,
            )),
            <get_binding::Response as Parameter>::ID => Ok(Self::GetBinding(
                get_binding::Response::from_le_stream_exact(stream)?,
            )),
            <get_binding_remote_node_id::Response as Parameter>::ID => {
                Ok(Self::GetBindingRemoteNodeId(
                    get_binding_remote_node_id::Response::from_le_stream_exact(stream)?,
                ))
            }
            <set_binding::Response as Parameter>::ID => Ok(Self::SetBinding(
                set_binding::Response::from_le_stream_exact(stream)?,
            )),
            <set_binding_remote_node_id::Response as Parameter>::ID => {
                Ok(Self::SetBindingRemoteNodeId(
                    set_binding_remote_node_id::Response::from_le_stream_exact(stream)?,
                ))
            }
            _ => Err(Decode::InvalidFrameId(id)),
        }
    }
}
