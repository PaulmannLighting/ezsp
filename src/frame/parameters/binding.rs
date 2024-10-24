//! Binding Frames

pub mod binding_is_active;
pub mod clear_binding_table;
pub mod delete_binding;
pub mod get_binding;
pub mod get_binding_remote_node_id;
pub mod handler;
pub mod set_binding;
pub mod set_binding_remote_node_id;

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
    /// Callback handlers.
    Handler(handler::Handler),
}
