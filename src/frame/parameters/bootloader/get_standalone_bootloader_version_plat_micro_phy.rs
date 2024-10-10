use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0091;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    bootloader_version: u16,
    node_plat: u8,
    node_micro: u8,
    node_phy: u8,
}

impl Response {
    #[must_use]
    pub const fn bootloader_version(&self) -> u16 {
        self.bootloader_version
    }

    #[must_use]
    pub const fn node_plat(&self) -> u8 {
        self.node_plat
    }

    #[must_use]
    pub const fn node_micro(&self) -> u8 {
        self.node_micro
    }

    #[must_use]
    pub const fn node_phy(&self) -> u8 {
        self.node_phy
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
