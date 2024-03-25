#[cfg(feature = "ashv2")]
mod ashv2;
mod ezsp;

#[cfg(feature = "ashv2")]
pub use ashv2::Ashv2;
pub use ezsp::{
    Binding, Bootloader, CertificateBasedKeyExchange, Configuration, Ezsp, Messaging, TrustCenter,
};
use le_stream::{FromLeBytes, ToLeBytes};
use std::fmt::Debug;

pub trait Transport {
    fn next_command<I, T>(&mut self, frame_id: I, parameters: T) -> Vec<u8>
    where
        I: Copy + Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
        T: ToLeBytes;
}
