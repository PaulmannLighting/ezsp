//! Security Frames

pub(crate) mod check_key_context;
pub(crate) mod clear_key_table;
pub(crate) mod clear_transient_link_keys;
pub(crate) mod erase_key_table_entry;
pub(crate) mod export_key;
pub(crate) mod export_link_key_by_eui;
pub(crate) mod export_link_key_by_index;
pub mod export_transient_key;
pub(crate) mod find_key_table_entry;
pub mod get_aps_key_info;
pub(crate) mod get_current_security_state;
pub(crate) mod get_network_key_info;
pub mod handler;
pub(crate) mod import_key;
pub(crate) mod import_link_key;
pub(crate) mod import_transient_key;
pub(crate) mod request_link_key;
pub(crate) mod send_trust_center_link_key;
pub(crate) mod set_initial_security_state;
pub(crate) mod update_tc_link_key;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
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
    GetNetworkKeyInfo(get_network_key_info::Command),
    ImportKey(import_key::Command),
    ImportLinkKey(import_link_key::Command),
    ImportTransientKey(import_transient_key::Command),
    RequestLinkKey(request_link_key::Command),
    SendTrustCenterLinkKey(send_trust_center_link_key::Command),
    SetInitialSecurityState(set_initial_security_state::Command),
    UpdateTcLinkKey(update_tc_link_key::Command),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    CheckKeyContext(check_key_context::Response),
    ClearKeyTable(clear_key_table::Response),
    ClearTransientLinkKeys(clear_transient_link_keys::Response),
    EraseKeyTableEntry(erase_key_table_entry::Response),
    ExportKey(export_key::Response),
    ExportLinkKeyByEui(export_link_key_by_eui::Response),
    ExportLinkKeyByIndex(export_link_key_by_index::Response),
    ExportTransientKeyByEui(export_transient_key::by_eui::Response),
    ExportTransientKeyByIndex(export_transient_key::by_index::Response),
    FindKeyTableEntry(find_key_table_entry::Response),
    GetApsKeyInfo(get_aps_key_info::Response),
    GetCurrentSecurityState(get_current_security_state::Response),
    GetNetworkKeyInfo(get_network_key_info::Response),
    ImportKey(import_key::Response),
    ImportLinkKey(import_link_key::Response),
    ImportTransientKey(import_transient_key::Response),
    RequestLinkKey(request_link_key::Response),
    SendTrustCenterLinkKey(send_trust_center_link_key::Response),
    SetInitialSecurityState(set_initial_security_state::Response),
    UpdateTcLinkKey(update_tc_link_key::Response),
    Handler(handler::Handler),
}
