//! Binding Frames

pub use self::clear_table::Response as ClearTable;
pub use self::delete::Response as Delete;
pub use self::get::Response as Get;
pub use self::get_remote_node_id::Response as GetRemoteNodeId;
pub use self::is_active::Response as IsActive;
pub use self::set::Response as Set;
pub use self::set_remote_node_id::Response as SetRemoteNodeId;

pub mod clear_table;
pub mod delete;
pub mod get;
pub mod get_remote_node_id;
pub mod handler;
pub mod is_active;
pub mod set;
pub mod set_remote_node_id;

crate::frame::parameters::command_enum!(
    Command,
    ClearTable(clear_table::Command),
    Delete(delete::Command),
    Get(get::Command),
    GetRemoteNodeId(get_remote_node_id::Command),
    IsActive(is_active::Command),
    Set(set::Command),
    SetRemoteNodeId(set_remote_node_id::Command),
);

crate::frame::parameters::parameter_enum!(
    Response,
    ClearTable,
    Delete,
    Get,
    GetRemoteNodeId,
    IsActive,
    Set,
    SetRemoteNodeId
);
