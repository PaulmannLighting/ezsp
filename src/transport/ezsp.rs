use std::future::Future;

pub use binding::Binding;
pub use bootloader::Bootloader;
pub use cbke::CertificateBasedKeyExchange;
pub use configuration::Configuration;
pub use green_power::{GreenPower, ProxyTable, SinkTable};
pub use messaging::Messaging;
pub use mfglib::Mfglib;
pub use networking::Networking;
pub use security::Security;
pub use token_interface::TokenInterface;
pub use trust_center::TrustCenter;
pub use utilities::Utilities;
pub use wwah::Wwah;
pub use zll::Zll;

use crate::frame::parameters::configuration::version;
use crate::Error;

mod binding;
mod bootloader;
mod cbke;
mod configuration;
mod green_power;
mod messaging;
mod mfglib;
mod networking;
mod security;
mod token_interface;
mod trust_center;
mod utilities;
mod wwah;
mod zll;

const MIN_NON_LEGACY_VERSION: u8 = 8;

/// A trait to represent implementors of the full `EZSP` protocol.
pub trait Ezsp:
    Binding
    + Bootloader
    + CertificateBasedKeyExchange
    + Configuration
    + GreenPower
    + Messaging
    + Mfglib
    + Networking
    + Security
    + TokenInterface
    + TrustCenter
    + Utilities
    + Wwah
    + Zll
{
    /// Negotiate the protocol version with the NCP.
    fn negotiate_version(
        &mut self,
        desired_protocol_version: u8,
    ) -> impl Future<Output = Result<version::Response, Error>> + Send;
}

impl<T> Ezsp for T
where
    T: Binding
        + Bootloader
        + CertificateBasedKeyExchange
        + Configuration
        + GreenPower
        + Messaging
        + Mfglib
        + Networking
        + Security
        + TokenInterface
        + TrustCenter
        + Utilities
        + Wwah
        + Zll
        + Send,
{
    async fn negotiate_version(
        &mut self,
        desired_protocol_version: u8,
    ) -> Result<version::Response, Error> {
        let response = self.legacy_version(desired_protocol_version).await?;

        if response.protocol_version() >= MIN_NON_LEGACY_VERSION {
            return self.version(response.protocol_version()).await;
        }

        Ok(response)
    }
}
