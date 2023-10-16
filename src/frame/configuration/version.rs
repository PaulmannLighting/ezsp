use crate::frame::header::LegacyHeader;
use crate::frame::LegacyFrame;
use std::io::Read;

const ID: u8 = 0x00;

/// The command allows the Host to specify the desired EZSP version
/// and must be sent before any other command.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: LegacyHeader,
    desired_protocol_version: u8,
}

impl Command {
    #[must_use]
    pub const fn new(sequence: u8, control: u8, desired_protocol_version: u8) -> Self {
        Self {
            header: LegacyHeader::for_frame::<ID>(sequence, control),
            desired_protocol_version,
        }
    }

    #[must_use]
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

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [desired_protocol_version]: [u8; 1] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            header,
            desired_protocol_version,
        })
    }
}

/// The response provides information about the firmware running on the NCP.
#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: LegacyHeader,
    protocol_version: u8,
    stack_type: u8,
    stack_version: u8,
}

impl Response {
    #[must_use]
    pub const fn new(
        sequence: u8,
        control: u8,
        protocol_version: u8,
        stack_type: u8,
        stack_version: u8,
    ) -> Self {
        Self {
            header: LegacyHeader::for_frame::<ID>(sequence, control),
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

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [protocol_version, stack_type, stack_version]: [u8; 3] = [0; 3];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            header,
            protocol_version,
            stack_type,
            stack_version,
        })
    }
}
