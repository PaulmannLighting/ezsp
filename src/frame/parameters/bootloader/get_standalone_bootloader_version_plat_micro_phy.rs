//! Detects if the standalone bootloader is installed, and if so returns the installed version.
//! If not return `0xffff`.
//!
//! A returned version of `0x1234` would indicate version 1.2 build 34.
//!
//! Also return the node's version of `PLAT`, `MICRO` and `PHY`.

use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0091;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

/// The response to a get standalone bootloader version plat micro phy command.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    bootloader_version: u16,
    node_plat: u8,
    node_micro: u8,
    node_phy: u8,
}

impl Response {
    // TODO: Where is `BOOTLOADER_INVALID_VERSION` defined?
    /// `BOOTLOADER_INVALID_VERSION` if the standalone bootloader is not present,
    /// or the version of the installed standalone bootloader.
    #[must_use]
    pub const fn bootloader_version(&self) -> u16 {
        self.bootloader_version
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
    const ID: Option<Self::Id> = Some(ID);
}
