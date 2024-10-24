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
