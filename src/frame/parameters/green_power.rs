//! Green Power Frames

pub mod handler;
pub(crate) mod proxy_table;
pub(crate) mod send;
pub(crate) mod sink_commission;
pub(crate) mod sink_table;
pub(crate) mod translation_table_clear;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    ProxyTable(proxy_table::Command),
    Send(send::Command),
    SinkCommission(sink_commission::Command),
    SinkTable(sink_table::Command),
    TranslationTableClear(translation_table_clear::Command),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    ProxyTable(proxy_table::Response),
    Send(send::Response),
    SinkCommission(sink_commission::Response),
    SinkTable(sink_table::Response),
    TranslationTableClear(translation_table_clear::Response),
    Handler(handler::Handler),
}
