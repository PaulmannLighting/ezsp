mod binding;
mod bootloader;
mod cbke;
mod configuration;
mod messaging;
mod trust_center;

use crate::frame::parameters::version;
pub use crate::transport::ezsp::binding::Binding;
pub use crate::transport::ezsp::bootloader::Bootloader;
pub use crate::transport::ezsp::cbke::CertificateBasedKeyExchange;
pub use crate::transport::ezsp::messaging::Messaging;
pub use crate::transport::ezsp::trust_center::TrustCenter;
use crate::Error;
pub use configuration::Configuration;
use std::future::Future;

const MIN_NON_LEGACY_VERSION: u8 = 8;

pub trait Ezsp:
    Binding + Bootloader + CertificateBasedKeyExchange + Configuration + Messaging + TrustCenter
{
    fn negotiate_version(
        &mut self,
        desired_protocol_version: u8,
    ) -> impl Future<Output = Result<version::Response, Error>>;
}

impl<T> Ezsp for T
where
    T: Binding + Bootloader + CertificateBasedKeyExchange + Configuration + Messaging + TrustCenter,
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
