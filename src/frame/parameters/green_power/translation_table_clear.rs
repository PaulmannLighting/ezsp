//! Parameters for the [`GreenPower::translation_table_clear`](crate::GreenPower::translation_table_clear) command.

crate::frame::parameters::frame!(
    0x010B,
    {},
    {} => GreenPower(green_power)::TranslationTableClear,
);
