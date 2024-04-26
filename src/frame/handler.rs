use crate::frame::parameters::{bootloader, cbke, mfglib, networking};

/// Possible callback responses, which are called "handler"s according to the EZSP documentation.
#[allow(clippy::large_enum_variant)]
pub enum Handler {
    BootloadTransmitComplete(bootloader::bootload_transmit_complete_handler::Handler),
    CalculateSmacs(cbke::calculate_smacs_handler::Handler),
    CalculateSmacs283k1(cbke::calculate_smacs_handler283k1::Handler),
    ChildJoin(networking::child_join_handler::Handler),
    DutyCycleHandler(networking::duty_cycle_handler::Handler),
    EnergyScanResult(networking::energy_scan_result_handler::Handler),
    MfglibRxHandler(mfglib::rx_handler::Handler),
    // TODO: implement all.
}
