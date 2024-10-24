use crate::frame::parameters::{
    binding, bootloader, cbke, green_power, messaging, mfglib, networking, security, trust_center,
    utilities, zll,
};

/// Possible callback responses, which are called "handler"s according to the EZSP documentation.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// Binding handlers.
    Binding(binding::handler::Handler),
    /// Bootloader handlers.
    Bootloader(bootloader::handler::Handler),
    /// Certificate-based key exchange handlers.
    Cbke(cbke::handler::Handler),
    /// Green Power handlers.
    GreenPower(green_power::handler::Handler),
    /// Messaging handlers.
    Messaging(messaging::handler::Handler),
    /// `Mfglib` handlers.
    MfgLib(mfglib::handler::Handler),
    /// Networking handlers.
    Networking(networking::handler::Handler),
    /// Security handlers.
    Security(security::handler::Handler),
    /// Trust Center handlers.
    TrustCenter(trust_center::handler::Handler),
    /// Utilities handlers.
    Utilities(utilities::handler::Handler),
    /// ZLL handlers.
    Zll(zll::handler::Handler),
}
