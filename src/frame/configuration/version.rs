use crate::frame::Parameters;
use std::array::IntoIter;
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

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 1>;

    fn into_iter(self) -> Self::IntoIter {
        [self.desired_protocol_version].into_iter()
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

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [self.protocol_version, self.stack_type, self.stack_version].into_iter()
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

#[cfg(test)]
mod tests {
    use super::Command;
    use crate::frame::LegacyFrame;

    #[test]
    fn test_command_serialization() {
        let command = Command::new(4);
        let frame = LegacyFrame::new(1, 0, command);
        let bytes: Vec<u8> = frame.into();
        assert_eq!(bytes, vec![0x01, 0x00, 0x00, 0x04]);
    }
}
