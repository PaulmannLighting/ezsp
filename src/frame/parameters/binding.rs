//! Binding Frames

pub mod clear_table;
pub mod delete;
pub mod get;
pub mod get_remote_node_id;
pub mod handler;
pub mod is_active;
pub mod set;
pub mod set_remote_node_id;

/// Response parameters for binding frames.
#[expect(variant_size_differences)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    /// Response parameters for [`Binding::clear_binding_table()`](crate::Binding::clear_table).
    ClearTable(clear_table::Response),
    /// Response parameters for [`Binding::delete_binding()`](crate::Binding::delete).
    Delete(delete::Response),
    /// Response parameters for [`Binding::get_binding()`](crate::Binding::get).
    Get(Box<get::Response>),
    /// Response parameters for [`Binding::get_binding_remote_node_id()`](crate::Binding::get_remote_node_id).
    GetRemoteNodeId(get_remote_node_id::Response),
    /// Response parameters for [`Binding::binding_is_active()`](crate::Binding::is_active).
    IsActive(is_active::Response),
    /// Response parameters for [`Binding::set_binding()`](crate::Binding::set).
    Set(set::Response),
    /// Response parameters for [`Binding::set_binding_remote_node_id()`](crate::Binding::set_remote_node_id).
    SetRemoteNodeId(set_remote_node_id::Response),
}
