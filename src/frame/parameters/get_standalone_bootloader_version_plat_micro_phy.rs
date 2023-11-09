use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0091;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    bootloader_version: u16,
    node_plat: u8,
    node_micro: u8,
    node_phy: u8,
}

impl Response {
    #[must_use]
    pub const fn new(bootloader_version: u16, node_plat: u8, node_micro: u8, node_phy: u8) -> Self {
        Self {
            bootloader_version,
            node_plat,
            node_micro,
            node_phy,
        }
    }

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
