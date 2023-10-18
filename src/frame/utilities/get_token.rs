use crate::ember::Status;
use crate::frame::Parameters;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{once, Chain, Once};

pub const ID: u16 = 0x000A;

/// Retrieves a token (8 bytes of non-volatile storage) from the Simulated EEPROM of the NCP.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    token_id: u8,
}

impl Command {
    #[must_use]
    pub const fn new(token_id: u8) -> Self {
        Self { token_id }
    }

    #[must_use]
    pub const fn token_id(&self) -> u8 {
        self.token_id
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 1>;

    fn into_iter(self) -> Self::IntoIter {
        self.token_id.to_be_bytes().into_iter()
    }
}

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [token_id] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self { token_id })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: Status,
    token_data: [u8; 8],
}

impl Response {
    #[must_use]
    pub const fn new(status: Status, token_data: [u8; 8]) -> Self {
        Self { status, token_data }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }

    #[must_use]
    pub const fn token_data(&self) -> &[u8] {
        &self.token_data
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = Chain<Once<Self::Item>, IntoIter<Self::Item, 8>>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.status.into()).chain(self.token_data)
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [status, token_data @ ..] = [0; 9];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            status: Status::try_from(status)?,
            token_data,
        })
    }
}
