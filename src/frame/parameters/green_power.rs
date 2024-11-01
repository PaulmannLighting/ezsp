//! Green Power Frames

pub mod handler;
pub mod proxy_table;
pub mod send;
pub mod sink_commission;
pub mod sink_table;
pub mod translation_table_clear;

/// Green Power response parameters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    /// Responses to proxy table related commands.
    ProxyTable(proxy_table::Response),
    /// Response to the `send` command.
    Send(send::Response),
    /// Response to the `sink_commission` command.
    SinkCommission(sink_commission::Response),
    /// Responses to sink table related commands.
    SinkTable(sink_table::Response),
    /// Response to the `translation_table_clear` command.
    TranslationTableClear(translation_table_clear::Response),
}
