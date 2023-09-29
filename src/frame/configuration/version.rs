use crate::frame::header::LegacyHeader;

pub struct Command {
    header: LegacyHeader,
    desired_protocol_version: u8,
}

pub struct Response {
    header: LegacyHeader,
    protocol_version: u8,
    stack_type: u8,
    stack_version: u8,
}
