//! Green Power Frames

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
