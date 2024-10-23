//! Green Power Frames

use crate::error::Decode;
use crate::frame::parsable::Parsable;
use crate::frame::Parameter;
use le_stream::FromLeStream;

pub mod handler;
pub mod proxy_table;
pub mod send;
pub mod sink_commission;
pub mod sink_table;
pub mod translation_table_clear;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    ProxyTable(proxy_table::Response),
    Send(send::Response),
    SinkCommission(sink_commission::Response),
    SinkTable(sink_table::Response),
    TranslationTableClear(translation_table_clear::Response),
    Handler(handler::Handler),
}

impl Parsable for Response {
    fn parse_from_le_stream<T>(id: u16, stream: T) -> Result<Self, Decode>
    where
        T: Iterator<Item = u8>,
    {
        match id {
            <proxy_table::get_entry::Response as Parameter>::ID => {
                Ok(Self::ProxyTable(proxy_table::Response::GetEntry(
                    proxy_table::get_entry::Response::from_le_stream_exact(stream)?,
                )))
            }
            <proxy_table::lookup::Response as Parameter>::ID => {
                Ok(Self::ProxyTable(proxy_table::Response::Lookup(
                    proxy_table::lookup::Response::from_le_stream_exact(stream)?,
                )))
            }
            <proxy_table::process_gp_pairing::Response as Parameter>::ID => {
                Ok(Self::ProxyTable(proxy_table::Response::ProcessGpPairing(
                    proxy_table::process_gp_pairing::Response::from_le_stream_exact(stream)?,
                )))
            }
            <send::Response as Parameter>::ID => {
                Ok(Self::Send(send::Response::from_le_stream_exact(stream)?))
            }
            <sink_commission::Response as Parameter>::ID => Ok(Self::SinkCommission(
                sink_commission::Response::from_le_stream_exact(stream)?,
            )),
            <sink_table::clear_all::Response as Parameter>::ID => {
                Ok(Self::SinkTable(sink_table::Response::ClearAll(
                    sink_table::clear_all::Response::from_le_stream_exact(stream)?,
                )))
            }
            <sink_table::find_or_allocate_entry::Response as Parameter>::ID => {
                Ok(Self::SinkTable(sink_table::Response::FindOrAllocateEntry(
                    sink_table::find_or_allocate_entry::Response::from_le_stream_exact(stream)?,
                )))
            }
            <sink_table::get_entry::Response as Parameter>::ID => {
                Ok(Self::SinkTable(sink_table::Response::GetEntry(
                    sink_table::get_entry::Response::from_le_stream_exact(stream)?,
                )))
            }
            <sink_table::init::Response as Parameter>::ID => {
                Ok(Self::SinkTable(sink_table::Response::Init(
                    sink_table::init::Response::from_le_stream_exact(stream)?,
                )))
            }
            <sink_table::lookup::Response as Parameter>::ID => {
                Ok(Self::SinkTable(sink_table::Response::Lookup(
                    sink_table::lookup::Response::from_le_stream_exact(stream)?,
                )))
            }
            <sink_table::number_of_active_entries::Response as Parameter>::ID => Ok(
                Self::SinkTable(sink_table::Response::NumberOfActiveEntries(
                    sink_table::number_of_active_entries::Response::from_le_stream_exact(stream)?,
                )),
            ),
            <sink_table::remove_entry::Response as Parameter>::ID => {
                Ok(Self::SinkTable(sink_table::Response::RemoveEntry(
                    sink_table::remove_entry::Response::from_le_stream_exact(stream)?,
                )))
            }
            <sink_table::set_entry::Response as Parameter>::ID => {
                Ok(Self::SinkTable(sink_table::Response::SetEntry(
                    sink_table::set_entry::Response::from_le_stream_exact(stream)?,
                )))
            }
            <sink_table::set_security_frame_counter::Response as Parameter>::ID => Ok(
                Self::SinkTable(sink_table::Response::SetSecurityFrameCounter(
                    sink_table::set_security_frame_counter::Response::from_le_stream_exact(stream)?,
                )),
            ),
            <translation_table_clear::Response as Parameter>::ID => {
                Ok(Self::TranslationTableClear(
                    translation_table_clear::Response::from_le_stream_exact(stream)?,
                ))
            }
            <handler::IncomingMessage as Parameter>::ID => {
                Ok(Self::Handler(handler::Handler::IncomingMessage(
                    handler::IncomingMessage::from_le_stream_exact(stream)?,
                )))
            }
            <handler::Sent as Parameter>::ID => Ok(Self::Handler(handler::Handler::Sent(
                handler::Sent::from_le_stream_exact(stream)?,
            ))),
            _ => Err(Decode::InvalidFrameId(id)),
        }
    }
}
