use std::fmt::{Debug, Display};

use le_stream::{FromLeBytes, ToLeBytes};

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
        + FromLeBytes
        + ToLeBytes;
    const ID: Self::Id;
}

/// Possible callback responses, which are called "handler"s according to the EZSP documentation.
#[allow(clippy::large_enum_variant)]
pub enum Handler {
    BootloadTransmitComplete(bootloader::bootload_transmit_complete_handler::Response),
    CalculateSmacs(cbke::calculate_smacs_handler::Response),
    CalculateSmacs283k1(cbke::calculate_smacs283k1::Response),
    MfglibRxHandler(mfglib::rx_handler::Handler),
    // TODO: implement all.
}
