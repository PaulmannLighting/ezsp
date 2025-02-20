//! The command allows the Host to specify the desired `EZSP` version
//! and must be sent before any other command.
//!
//! The response provides information about the firmware running on the NCP.

use std::fmt::Debug;

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ezsp::StackVersion;
use crate::frame::Parameter;

const ID: u16 = 0x0000;

/// The response provides information about the firmware running on the NCP.
#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    desired_protocol_version: u8,
}

impl Command {
    #[must_use]
    pub const fn new(desired_protocol_version: u8) -> Self {
        Self {
            desired_protocol_version,
        }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// The response provides information about the firmware running on the NCP.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    protocol_version: u8,
    stack_type: u8,
    stack_version: u16,
}

impl Response {
    /// The EZSP version the NCP is using.
    #[must_use]
    pub const fn protocol_version(&self) -> u8 {
        self.protocol_version
    }

    /// The type of stack running on the NCP (2).
    #[must_use]
    pub const fn stack_type(&self) -> u8 {
        self.stack_type
    }

    /// The version number of the stack.
    #[must_use]
    pub const fn stack_version(&self) -> StackVersion {
        StackVersion(self.stack_version)
    }
}

impl Parameter for Response {
    const ID: u16 = ID;
}
