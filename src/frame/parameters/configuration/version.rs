use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};
use std::fmt::Debug;

const ID: u8 = 0x00;

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

    #[must_use]
    pub const fn desired_protocol_version(&self) -> u8 {
        self.desired_protocol_version
    }
}

impl Parameter for Command {
    type Id = u8;
    const ID: Self::Id = ID;
}

/// The response to a version command.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    protocol_version: u8,
    stack_type: u8,
    stack_version: u16,
}

impl Response {
    #[must_use]
    pub const fn protocol_version(&self) -> u8 {
        self.protocol_version
    }

    #[must_use]
    pub const fn stack_type(&self) -> u8 {
        self.stack_type
    }

    #[must_use]
    pub const fn stack_version(&self) -> u16 {
        self.stack_version
    }
}

impl Parameter for Response {
    type Id = u8;
    const ID: Self::Id = ID;
}
