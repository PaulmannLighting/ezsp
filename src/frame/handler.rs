use crate::frame::parameters::{bootloader, cbke, mfglib};

/// Possible callback responses, which are called "handler"s according to the EZSP documentation.
#[allow(clippy::large_enum_variant)]
pub enum Handler {
    BootloadTransmitComplete(bootloader::bootload_transmit_complete_handler::Handler),
    CalculateSmacs(cbke::calculate_smacs_handler::Handler),
    CalculateSmacs283k1(cbke::calculate_smacs_handler283k1::Handler),
    MfglibRxHandler(mfglib::rx_handler::Handler),
    // TODO: implement all.
}
