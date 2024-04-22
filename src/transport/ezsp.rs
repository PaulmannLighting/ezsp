use std::future::Future;

pub use binding::Binding;
pub use bootloader::Bootloader;
pub use cbke::CertificateBasedKeyExchange;
pub use configuration::Configuration;
pub use green_power::GreenPower;
pub use messaging::Messaging;
pub use networking::Networking;
pub use security::Security;
pub use trust_center::TrustCenter;
pub use utilities::Utilities;

use crate::frame::parameters::configuration::version;
use crate::Error;

mod binding;
mod bootloader;
mod cbke;
mod configuration;
mod green_power;
mod messaging;
mod networking;
mod security;
mod trust_center;
mod utilities;

const MIN_NON_LEGACY_VERSION: u8 = 8;

pub trait Ezsp:
    Binding
    + Bootloader
    + CertificateBasedKeyExchange
    + Configuration
    + GreenPower
    + Messaging
    + Networking
    + Security
    + TrustCenter
    + Utilities
{
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
        + Networking
        + Security
        + TrustCenter
        + Utilities
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
