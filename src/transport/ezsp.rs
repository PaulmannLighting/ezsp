use std::future::Future;

pub use binding::Binding;
pub use bootloader::Bootloader;
pub use cbke::CertificateBasedKeyExchange;
pub use configuration::Configuration;
pub use messaging::Messaging;
pub use networking::Networking;
pub use security::Security;
pub use trust_center::TrustCenter;

use crate::Error;
use crate::frame::parameters::version;

mod binding;
mod bootloader;
mod cbke;
mod configuration;
mod messaging;
mod networking;
mod security;
mod trust_center;

const MIN_NON_LEGACY_VERSION: u8 = 8;

pub trait Ezsp:
    Binding
    + Bootloader
    + CertificateBasedKeyExchange
    + Configuration
    + Messaging
    + Security
    + TrustCenter
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
        + Messaging
        + Networking
        + Security
        + TrustCenter
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
