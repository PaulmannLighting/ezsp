mod binding;
mod bootloader;
mod cbke;
mod configuration;
mod messaging;
mod networking;
mod trust_center;

use crate::frame::parameters::version;
use crate::Error;
pub use binding::Binding;
pub use bootloader::Bootloader;
pub use cbke::CertificateBasedKeyExchange;
pub use configuration::Configuration;
pub use messaging::Messaging;
pub use networking::Networking;
use std::future::Future;
pub use trust_center::TrustCenter;

const MIN_NON_LEGACY_VERSION: u8 = 8;

pub trait Ezsp:
    Binding + Bootloader + CertificateBasedKeyExchange + Configuration + Messaging + TrustCenter
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
