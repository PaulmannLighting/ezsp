//! Enumeration containing all possible `EZSP` response parameters.

use crate::parameters::{
    binding, bootloader, cbke, configuration, green_power, messaging, mfglib, networking, security,
    token_interface, trust_center, utilities, wwah, zll,
};

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
