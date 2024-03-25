mod binding;
mod bootloader;
mod cbke;
mod configuration;
mod messaging;
mod trust_center;

pub use crate::transport::ezsp::binding::Binding;
pub use crate::transport::ezsp::bootloader::Bootloader;
pub use crate::transport::ezsp::cbke::CertificateBasedKeyExchange;
pub use crate::transport::ezsp::messaging::Messaging;
pub use crate::transport::ezsp::trust_center::TrustCenter;
pub use configuration::Configuration;

pub trait Ezsp:
    Binding + Bootloader + CertificateBasedKeyExchange + Configuration + Messaging + TrustCenter
{
}

impl<T> Ezsp for T where
    T: Binding + Bootloader + CertificateBasedKeyExchange + Configuration + Messaging + TrustCenter
{
}
