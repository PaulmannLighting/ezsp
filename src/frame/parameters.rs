//! Definitions of parameter structures used in the `Ember ZNet Serial Protocol` (`EZSP`).

pub mod binding;
pub mod bootloader;
pub mod cbke;
pub mod configuration;
pub mod green_power;
pub mod messaging;
pub mod mfglib;
pub mod networking;
pub mod security;
pub(crate) mod token_interface;
pub mod trust_center;
pub mod utilities;
pub mod wwah;
pub mod zll;

/// `EZSP` command parameters.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum Command {
    /// Command parameters for binding frames.
    Binding(binding::Command),
    /// Command parameters for bootloader frames.
    Bootloader(bootloader::Command),
    /// Command parameters for certificate-based key exchange (CBKE) frames.
    Cbke(cbke::Command),
    /// Command parameters for configuration frames.
    Configuration(configuration::Command),
    /// Command parameters for green power frames.
    GreenPower(green_power::Command),
    /// Command parameters for messaging frames.
    Messaging(messaging::Command),
    /// Command parameters for Manufacturing Test Library (`MfgLib`) frames.
    MfgLib(mfglib::Command),
    /// Command parameters for networking frames.
    Networking(networking::Command),
    /// Command parameters for security frames.
    Security(security::Command),
    /// Command parameters for token interface frames.
    TokenInterface(token_interface::Command),
    /// Command parameters for trust center frames.
    TrustCenter(trust_center::Command),
    /// Command parameters for utilities frames.
    Utilities(utilities::Command),
    /// Command parameters for Wireless Workgroup for Home Automation (WWAH) frames.
    Wwah(wwah::Command),
    /// Command parameters for Zigbee Light Link (ZLL) frames.
    Zll(zll::Command),
}

/// `EZSP` response parameters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    /// Response parameters for binding frames.
    Binding(binding::Response),
    /// Response parameters for bootloader frames.
    Bootloader(bootloader::Response),
    /// Response parameters for certificate-based key exchange (CBKE) frames.
    Cbke(cbke::Response),
    /// Response parameters for configuration frames.
    Configuration(configuration::Response),
    /// Response parameters for green power frames.
    GreenPower(green_power::Response),
    /// Response parameters for messaging frames.
    Messaging(messaging::Response),
    /// Response parameters for Manufacturing Test Library (`MfgLib`) frames.
    MfgLib(mfglib::Response),
    /// Response parameters for networking frames.
    Networking(networking::Response),
    /// Response parameters for security frames.
    Security(security::Response),
    /// Response parameters for token interface frames.
    TokenInterface(token_interface::Response),
    /// Response parameters for trust center frames.
    TrustCenter(trust_center::Response),
    /// Response parameters for utilities frames.
    Utilities(utilities::Response),
    /// Response parameters for Wireless Workgroup for Home Automation (WWAH) frames.
    Wwah(wwah::Response),
    /// Response parameters for Zigbee Light Link (ZLL) frames.
    Zll(zll::Response),
}
