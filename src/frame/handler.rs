use crate::frame::parameters::{binding, bootloader, cbke, mfglib, networking};

/// Possible callback responses, which are called "handler"s according to the EZSP documentation.
#[allow(clippy::large_enum_variant)]
pub enum Handler {
    Binding(binding::handler::Handler),
    Networking(networking::handler::Handler),
    // TODO: implement all.
}
