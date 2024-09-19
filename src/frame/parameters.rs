use std::fmt::{Debug, Display};

use le_stream::{FromLeStream, ToLeStream};

use crate::Control;

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

pub trait Parameter: Send + Sync {
    type Id: Copy
        + Debug
        + Display
        + Eq
        + Send
        + Sync
        + From<Control>
        + Into<Control>
        + Into<u16>
        + FromLeStream
        + ToLeStream;
    const ID: Self::Id;
}
