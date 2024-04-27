use crate::frame::parameters::{
    binding, bootloader, cbke, green_power, messaging, mfglib, networking, security, trust_center,
    utilities, zll,
};

/// Possible callback responses, which are called "handler"s according to the EZSP documentation.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    Binding(binding::handler::Handler),
    Bootloader(bootloader::handler::Handler),
    CertificateBasedKeyExchange(cbke::handler::Handler),
    GreenPower(green_power::handler::Handler),
    Messaging(messaging::handler::Handler),
    MfgLib(mfglib::handler::Handler),
    Networking(networking::handler::Handler),
    Security(security::handler::Handler),
    TrustCenter(trust_center::handler::Handler),
    Utilities(utilities::handler::Handler),
    Zll(zll::handler::Handler),
}
