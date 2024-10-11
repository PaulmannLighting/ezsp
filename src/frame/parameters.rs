use le_stream::{FromLeStream, ToLeStream};
use std::fmt::{Debug, Display, LowerHex, UpperHex};
use std::hash::Hash;

pub mod binding;
pub mod bootloader;
pub mod cbke;
pub mod configuration;
pub mod green_power;
pub mod messaging;
pub mod mfglib;
pub mod networking;
pub mod security;
pub mod token_interface;
pub mod trust_center;
pub mod utilities;
pub mod wwah;
pub mod zll;

pub trait Parameter: Debug + Send + Sync {
    type Id: Copy
        + Debug
        + Display
        + Eq
        + Into<u16>
        + LowerHex
        + UpperHex
        + FromLeStream
        + ToLeStream;
    const ID: Self::Id;
}
