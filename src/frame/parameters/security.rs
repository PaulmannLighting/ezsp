//! Security Frames

use le_stream::FromLeStream;

use crate::error::Decode;
use crate::frame::parsable::Parsable;
use crate::frame::Parameter;

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

impl Parsable for Response {
    fn parse_from_le_stream<T>(id: u16, stream: T) -> Result<Self, Decode>
    where
        T: Iterator<Item = u8>,
    {
        match id {
            <check_key_context::Response as Parameter>::ID => Ok(Self::CheckKeyContext(
                check_key_context::Response::from_le_stream_exact(stream)?,
            )),
            <clear_key_table::Response as Parameter>::ID => Ok(Self::ClearKeyTable(
                clear_key_table::Response::from_le_stream_exact(stream)?,
            )),
            <clear_transient_link_keys::Response as Parameter>::ID => {
                Ok(Self::ClearTransientLinkKeys(
                    clear_transient_link_keys::Response::from_le_stream_exact(stream)?,
                ))
            }
            <erase_key_table_entry::Response as Parameter>::ID => Ok(Self::EraseKeyTableEntry(
                erase_key_table_entry::Response::from_le_stream_exact(stream)?,
            )),
            <export_key::Response as Parameter>::ID => Ok(Self::ExportKey(
                export_key::Response::from_le_stream_exact(stream)?,
            )),
            <export_link_key_by_eui::Response as Parameter>::ID => Ok(Self::ExportLinkKeyByEui(
                export_link_key_by_eui::Response::from_le_stream_exact(stream)?,
            )),
            <export_link_key_by_index::Response as Parameter>::ID => {
                Ok(Self::ExportLinkKeyByIndex(
                    export_link_key_by_index::Response::from_le_stream_exact(stream)?,
                ))
            }
            <export_transient_key::by_eui::Response as Parameter>::ID => {
                Ok(Self::ExportTransientKeyByEui(
                    export_transient_key::by_eui::Response::from_le_stream_exact(stream)?,
                ))
            }
            <export_transient_key::by_index::Response as Parameter>::ID => {
                Ok(Self::ExportTransientKeyByIndex(
                    export_transient_key::by_index::Response::from_le_stream_exact(stream)?,
                ))
            }
            <find_key_table_entry::Response as Parameter>::ID => Ok(Self::FindKeyTableEntry(
                find_key_table_entry::Response::from_le_stream_exact(stream)?,
            )),
            <get_aps_key_info::Response as Parameter>::ID => Ok(Self::GetApsKeyInfo(
                get_aps_key_info::Response::from_le_stream_exact(stream)?,
            )),
            <get_current_security_state::Response as Parameter>::ID => {
                Ok(Self::GetCurrentSecurityState(
                    get_current_security_state::Response::from_le_stream_exact(stream)?,
                ))
            }
            <get_network_key_info::Response as Parameter>::ID => Ok(Self::GetNetworkKeyInfo(
                get_network_key_info::Response::from_le_stream_exact(stream)?,
            )),
            <import_key::Response as Parameter>::ID => Ok(Self::ImportKey(
                import_key::Response::from_le_stream_exact(stream)?,
            )),
            <import_link_key::Response as Parameter>::ID => Ok(Self::ImportLinkKey(
                import_link_key::Response::from_le_stream_exact(stream)?,
            )),
            <import_transient_key::Response as Parameter>::ID => Ok(Self::ImportTransientKey(
                import_transient_key::Response::from_le_stream_exact(stream)?,
            )),
            <request_link_key::Response as Parameter>::ID => Ok(Self::RequestLinkKey(
                request_link_key::Response::from_le_stream_exact(stream)?,
            )),
            <send_trust_center_link_key::Response as Parameter>::ID => {
                Ok(Self::SendTrustCenterLinkKey(
                    send_trust_center_link_key::Response::from_le_stream_exact(stream)?,
                ))
            }
            <set_initial_security_state::Response as Parameter>::ID => {
                Ok(Self::SetInitialSecurityState(
                    set_initial_security_state::Response::from_le_stream_exact(stream)?,
                ))
            }
            <update_tc_link_key::Response as Parameter>::ID => Ok(Self::UpdateTcLinkKey(
                update_tc_link_key::Response::from_le_stream_exact(stream)?,
            )),
            <handler::SwitchNetworkKey as Parameter>::ID => {
                Ok(Self::Handler(handler::Handler::SwitchNetworkKey(
                    handler::SwitchNetworkKey::from_le_stream_exact(stream)?,
                )))
            }
            <handler::ZigbeeKeyEstablishment as Parameter>::ID => {
                Ok(Self::Handler(handler::Handler::ZigbeeKeyEstablishment(
                    handler::ZigbeeKeyEstablishment::from_le_stream_exact(stream)?,
                )))
            }
            _ => Err(Decode::InvalidFrameId(id)),
        }
    }
}
