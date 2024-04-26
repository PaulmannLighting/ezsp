use crate::frame::parameters::{binding, bootloader, cbke, mfglib, networking};

/// Possible callback responses, which are called "handler"s according to the EZSP documentation.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    Binding(binding::handler::Handler),
    Networking(networking::handler::Handler),
    Bootloader(bootloader::handler::Handler),
    // TODO: implement all.
}
