use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::iter::{once, Chain, Once};

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
    type IntoIter = Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.desired_protocol_version)
    }
}

impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Ok(Self {
            desired_protocol_version: src.read_num_le()?,
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
    type IntoIter = Chain<Chain<Once<Self::Item>, Once<Self::Item>>, Once<Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.protocol_version)
            .chain(once(self.stack_type))
            .chain(once(self.stack_version))
    }
}

impl Readable for Response {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let [protocol_version, stack_type, stack_version] = src.read_array_exact()?;
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
    use crate::read_write::Writable;

    #[test]
    fn test_command_serialization() {
        let command = Command::new(4);
        let mut buffer = Vec::new();
        command
            .write_to(&mut buffer)
            .expect("Could not write frame.");
        assert_eq!(buffer, vec![0x04]);
    }
}
