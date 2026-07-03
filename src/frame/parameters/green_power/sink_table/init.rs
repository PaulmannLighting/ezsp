//! Parameters for the [`GreenPower::translation_table_clear`](crate::GreenPower::translation_table_clear) command.

crate::frame::parameters::frame!(
    0x0070,
    {},
    {} => GreenPower(green_power)::SinkTable(sink_table)::Init,
);
