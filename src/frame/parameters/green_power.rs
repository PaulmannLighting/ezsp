//! Green Power Frames

pub use self::proxy_table::Response as ProxyTable;
pub use self::send::Response as Send;
pub use self::sink_commission::Response as SinkCommission;
pub use self::sink_table::Response as SinkTable;
pub use self::translation_table_clear::Response as TranslationTableClear;

pub mod handler;
pub mod proxy_table;
pub mod send;
pub mod sink_commission;
pub mod sink_table;
pub mod translation_table_clear;

crate::frame::parameters::command_enum!(
    Command,
    ProxyTable(proxy_table::Command),
    Send(send::Command),
    SinkCommission(sink_commission::Command),
    SinkTable(sink_table::Command),
    TranslationTableClear(translation_table_clear::Command),
);

crate::frame::parameters::parameter_enum!(
    Response,
    ProxyTable,
    Send,
    SinkCommission,
    SinkTable,
    TranslationTableClear
);
