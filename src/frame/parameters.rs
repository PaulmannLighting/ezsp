//! This module contains the definition of the parameters used in the Ember ZNet protocol.

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

pub trait Parameter: Debug + Send
where
    <Self as Parameter>::Id: Copy
        + Debug
        + Display
        + Eq
        + Hash
        + Into<u16>
        + LowerHex
        + UpperHex
        + Send
        + FromLeStream
        + ToLeStream,
{
    type Id;
    const ID: Self::Id;
}
