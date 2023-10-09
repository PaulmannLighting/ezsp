use crate::frame::header::LegacyHeader;

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
}

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
}
