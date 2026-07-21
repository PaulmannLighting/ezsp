//! Security Frames

pub use self::check_key_context::Response as CheckKeyContext;
pub use self::clear_key_table::Response as ClearKeyTable;
pub use self::clear_transient_link_keys::Response as ClearTransientLinkKeys;
pub use self::erase_key_table_entry::Response as EraseKeyTableEntry;
pub use self::export_key::Response as ExportKey;
pub use self::export_link_key_by_eui::Response as ExportLinkKeyByEui;
pub use self::export_link_key_by_index::Response as ExportLinkKeyByIndex;
pub use self::export_transient_key::by_eui::Response as ExportTransientKeyByEui;
pub use self::export_transient_key::by_index::Response as ExportTransientKeyByIndex;
pub use self::find_key_table_entry::Response as FindKeyTableEntry;
pub use self::get_aps_key_info::Response as GetApsKeyInfo;
pub use self::get_current_security_state::Response as GetCurrentSecurityState;
pub use self::get_key::Response as GetKey;
pub use self::get_network_key_info::Response as GetNetworkKeyInfo;
pub use self::import_key::Response as ImportKey;
pub use self::import_link_key::Response as ImportLinkKey;
pub use self::import_transient_key::Response as ImportTransientKey;
pub use self::request_link_key::Response as RequestLinkKey;
pub use self::send_trust_center_link_key::Response as SendTrustCenterLinkKey;
pub use self::set_initial_security_state::Response as SetInitialSecurityState;
pub use self::update_tc_link_key::Response as UpdateTcLinkKey;

pub mod check_key_context;
pub mod clear_key_table;
pub mod clear_transient_link_keys;
pub mod erase_key_table_entry;
pub mod export_key;
pub mod export_link_key_by_eui;
pub mod export_link_key_by_index;
pub mod export_transient_key;
pub mod find_key_table_entry;
pub mod get_aps_key_info;
pub mod get_current_security_state;
pub mod get_key;
pub mod get_network_key_info;
pub mod handler;
pub mod import_key;
pub mod import_link_key;
pub mod import_transient_key;
pub mod request_link_key;
pub mod send_trust_center_link_key;
pub mod set_initial_security_state;
pub mod update_tc_link_key;

crate::frame::parameters::command_enum!(
    Command,
    CheckKeyContext(check_key_context::Command),
    ClearKeyTable(clear_key_table::Command),
    ClearTransientLinkKeys(clear_transient_link_keys::Command),
    EraseKeyTableEntry(erase_key_table_entry::Command),
    ExportKey(export_key::Command),
    ExportLinkKeyByEui(export_link_key_by_eui::Command),
    ExportLinkKeyByIndex(export_link_key_by_index::Command),
    ExportTransientKeyByEui(export_transient_key::by_eui::Command),
    ExportTransientKeyByIndex(export_transient_key::by_index::Command),
    FindKeyTableEntry(find_key_table_entry::Command),
    GetApsKeyInfo(get_aps_key_info::Command),
    GetCurrentSecurityState(get_current_security_state::Command),
    GetKey(get_key::Command),
    GetNetworkKeyInfo(get_network_key_info::Command),
    ImportKey(import_key::Command),
    ImportLinkKey(import_link_key::Command),
    ImportTransientKey(import_transient_key::Command),
    RequestLinkKey(request_link_key::Command),
    SendTrustCenterLinkKey(send_trust_center_link_key::Command),
    SetInitialSecurityState(set_initial_security_state::Command),
    UpdateTcLinkKey(update_tc_link_key::Command),
);

crate::frame::parameters::parameter_enum!(
    Response,
    CheckKeyContext,
    ClearKeyTable,
    ClearTransientLinkKeys,
    EraseKeyTableEntry,
    ExportKey,
    ExportLinkKeyByEui,
    ExportLinkKeyByIndex,
    ExportTransientKeyByEui,
    ExportTransientKeyByIndex,
    FindKeyTableEntry,
    GetApsKeyInfo,
    GetCurrentSecurityState,
    GetKey,
    GetNetworkKeyInfo,
    ImportKey,
    ImportLinkKey,
    ImportTransientKey,
    RequestLinkKey,
    SendTrustCenterLinkKey,
    SetInitialSecurityState,
    UpdateTcLinkKey
);
