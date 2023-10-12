use crate::frame::header::LegacyHeader;
use crate::frame::{Frame, LegacyFrame};

const ID: u8 = 0x00;

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: LegacyHeader,
    desired_protocol_version: u8,
}

impl Command {
    pub const fn new(sequence: u8, control: u8, desired_protocol_version: u8) -> Self {
        Self {
            header: Self::make_header(sequence, control),
            desired_protocol_version,
        }
    }

    pub const fn desired_protocol_version(&self) -> u8 {
        self.desired_protocol_version
    }
}

impl LegacyFrame<ID> for Command {
    type Parameters = [u8; 1];

    fn header(&self) -> &LegacyHeader {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([self.desired_protocol_version])
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
        sequence: u8,
        control: u8,
        protocol_version: u8,
        stack_type: u8,
        stack_version: u8,
    ) -> Self {
        Self {
            header: Self::make_header(sequence, control),
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
    type Parameters = [u8; 3];

    fn header(&self) -> &LegacyHeader {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([self.protocol_version, self.stack_type, self.stack_version])
    }
}
