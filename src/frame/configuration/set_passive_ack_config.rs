use crate::ember_status::EmberStatus;
use crate::frame::Parameters;
use std::array::IntoIter;
use std::io::Read;

pub const ID: u16 = 0x0105;

/// Allows the Host to control the broadcast behaviour of a routing device used by the NCP.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    config: u8,
    min_acks_needed: u8,
}

impl Command {
    #[must_use]
    pub const fn new(config: u8, min_acks_needed: u8) -> Self {
        Self {
            config,
            min_acks_needed,
        }
    }

    #[must_use]
    pub const fn config(&self) -> u8 {
        self.config
    }

    #[must_use]
    pub const fn min_acks_needed(&self) -> u8 {
        self.min_acks_needed
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [self.config, self.min_acks_needed].into_iter()
    }
}

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [config, min_acks_needed] = [0; 2];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            config,
            min_acks_needed,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: EmberStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 1>;

    fn into_iter(self) -> Self::IntoIter {
        [self.status.into()].into_iter()
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [status] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            status: EmberStatus::try_from(status)?,
        })
    }
}
