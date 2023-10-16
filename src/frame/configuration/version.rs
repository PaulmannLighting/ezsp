use crate::frame::Parameters;
use std::io::Read;

pub const ID: u8 = 0x00;

/// The command allows the Host to specify the desired EZSP version
/// and must be sent before any other command.
#[derive(Debug, Eq, PartialEq)]
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

impl From<Command> for Vec<u8> {
    fn from(command: Command) -> Self {
        vec![command.desired_protocol_version]
    }
}

impl Parameters<u8> for Command {
    const FRAME_ID: u8 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [desired_protocol_version] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            desired_protocol_version,
        })
    }
}

/// The response provides information about the firmware running on the NCP.
#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    protocol_version: u8,
    stack_type: u8,
    stack_version: u8,
}

impl Response {
    #[must_use]
    pub const fn new(protocol_version: u8, stack_type: u8, stack_version: u8) -> Self {
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
    pub const fn stack_version(&self) -> u8 {
        self.stack_version
    }
}

impl From<Response> for Vec<u8> {
    fn from(response: Response) -> Self {
        vec![
            response.protocol_version,
            response.stack_type,
            response.stack_version,
        ]
    }
}

impl Parameters<u8> for Response {
    const FRAME_ID: u8 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [protocol_version, stack_type, stack_version] = [0; 3];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            protocol_version,
            stack_type,
            stack_version,
        })
    }
}
