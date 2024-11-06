//! Parameters for the [`Bootloader::get_standalone_bootloader_version_plat_micro_phy()`](crate::Bootloader::get_standalone_bootloader_version_plat_micro_phy)
//! command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x0091;
const BOOTLOADER_INVALID_VERSION: u16 = 0xFFFF;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    bootloader_version: u16,
    node_plat: u8,
    node_micro: u8,
    node_phy: u8,
}

impl Response {
    /// `None` if the standalone bootloader is not present,
    /// or the version of the installed standalone bootloader.
    #[must_use]
    pub const fn bootloader_version(&self) -> Option<u16> {
        if self.bootloader_version == BOOTLOADER_INVALID_VERSION {
            None
        } else {
            Some(self.bootloader_version)
        }
    }

    /// The value of `PLAT` on the node.
    #[must_use]
    pub const fn node_plat(&self) -> u8 {
        self.node_plat
    }

    /// The value of `MICRO` on the node.
    #[must_use]
    pub const fn node_micro(&self) -> u8 {
        self.node_micro
    }

    /// The value of `PHY` on the node.
    #[must_use]
    pub const fn node_phy(&self) -> u8 {
        self.node_phy
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
