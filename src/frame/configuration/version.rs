use crate::frame::header::LegacyHeader;
use crate::frame::{Frame, LegacyFrame};

const ID: u8 = 0x00;

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: LegacyHeader,
    desired_protocol_version: u8,
}

impl Command {
    pub const fn new(header: LegacyHeader, desired_protocol_version: u8) -> Self {
        Self {
            header,
            desired_protocol_version,
        }
    }

    pub const fn desired_protocol_version(&self) -> u8 {
        self.desired_protocol_version
    }
}

impl LegacyFrame<ID> for Command {
    fn header(&self) -> &LegacyHeader {
        &self.header
    }

    fn parameters(&self) -> Vec<u8> {
        vec![self.desired_protocol_version]
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: LegacyHeader,
    protocol_version: u8,
    stack_type: u8,
    stack_version: u8,
}

impl Response {
    pub const fn new(
        header: LegacyHeader,
        protocol_version: u8,
        stack_type: u8,
        stack_version: u8,
    ) -> Self {
        Self {
            header,
            protocol_version,
            stack_type,
            stack_version,
        }
    }

    pub const fn protocol_version(&self) -> u8 {
        self.protocol_version
    }

    pub const fn stack_type(&self) -> u8 {
        self.stack_type
    }

    pub const fn stack_version(&self) -> u8 {
        self.stack_version
    }
}

impl LegacyFrame<ID> for Response {
    fn header(&self) -> &LegacyHeader {
        &self.header
    }

    fn parameters(&self) -> Vec<u8> {
        vec![self.protocol_version, self.stack_type, self.stack_version]
    }
}
