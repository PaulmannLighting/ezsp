//! Security Frames

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
pub mod get_network_key_info;
pub mod handler;
pub mod import_key;
pub mod import_link_key;
pub mod import_transient_key;
pub mod request_link_key;
pub mod send_trust_center_link_key;
pub mod set_initial_security_state;
pub mod update_tc_link_key;

/// Security frame response parameters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    /// Response parameters of the `check_key_context` command.
    CheckKeyContext(check_key_context::Response),
    /// Response parameters of the `clear_key_table` command.
    ClearKeyTable(clear_key_table::Response),
    /// Response parameters of the `clear_transient_link_keys` command.
    ClearTransientLinkKeys(clear_transient_link_keys::Response),
    /// Response parameters of the `erase_key_table_entry` command.
    EraseKeyTableEntry(erase_key_table_entry::Response),
    /// Response parameters of the `export_key` command.
    ExportKey(export_key::Response),
    /// Response parameters of the `export_link_key_by_eui` command.
    ExportLinkKeyByEui(export_link_key_by_eui::Response),
    /// Response parameters of the `export_link_key_by_index` command.
    ExportLinkKeyByIndex(export_link_key_by_index::Response),
    /// Response parameters of the `export_transient_key` command.
    ExportTransientKeyByEui(export_transient_key::by_eui::Response),
    /// Response parameters of the `export_transient_key` command.
    ExportTransientKeyByIndex(export_transient_key::by_index::Response),
    /// Response parameters of the `find_key_table_entry` command.
    FindKeyTableEntry(find_key_table_entry::Response),
    /// Response parameters of the `get_aps_key_info` command.
    GetApsKeyInfo(get_aps_key_info::Response),
    /// Response parameters of the `get_current_security_state` command.
    GetCurrentSecurityState(get_current_security_state::Response),
    /// Response parameters of the `get_network_key_info` command.
    GetNetworkKeyInfo(get_network_key_info::Response),
    /// Response parameters of the `handler` command.
    ImportKey(import_key::Response),
    /// Response parameters of the `import_link_key` command.
    ImportLinkKey(import_link_key::Response),
    /// Response parameters of the `import_transient_key` command.
    ImportTransientKey(import_transient_key::Response),
    /// Response parameters of the `request_link_key` command.
    RequestLinkKey(request_link_key::Response),
    /// Response parameters of the `send_trust_center_link_key` command.
    SendTrustCenterLinkKey(send_trust_center_link_key::Response),
    /// Response parameters of the `set_initial_security_state` command.
    SetInitialSecurityState(set_initial_security_state::Response),
    /// Response parameters of the `update_tc_link_key` command.
    UpdateTcLinkKey(update_tc_link_key::Response),
}
