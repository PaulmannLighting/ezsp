use crate::frame::Parameter;
use le_stream::derive::{FromLeBytes, ToLeBytes};
use std::fmt::Debug;

pub const ID: u8 = 0x00;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
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

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    protocol_version: u8,
    stack_type: u8,
    stack_version: u16,
}

impl Response {
    #[must_use]
    pub const fn new(protocol_version: u8, stack_type: u8, stack_version: u16) -> Self {
        Self {
            protocol_version,
            stack_type,
            stack_version,
        }
    }

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

impl Parameter<u8> for Response {
    const ID: u8 = ID;
}

impl Parameter<u16> for Response {
    const ID: u16 = ID as u16;
}
