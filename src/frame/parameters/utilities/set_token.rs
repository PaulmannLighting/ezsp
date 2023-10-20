use crate::ember::Status;
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{once, Chain, Once};

pub const ID: u16 = 0x0009;
const TOKEN_DATA_SIZE: usize = 8;

/// Sets a token (8 bytes of non-volatile storage) in the Simulated EEPROM of the NCP.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    token_id: u8,
    token_data: [u8; TOKEN_DATA_SIZE],
}

impl Command {
    #[must_use]
    pub const fn new(token_id: u8, token_data: [u8; TOKEN_DATA_SIZE]) -> Self {
        Self {
            token_id,
            token_data,
        }
    }

    #[must_use]
    pub const fn token_id(&self) -> u8 {
        self.token_id
    }

    #[must_use]
    pub const fn token_data(&self) -> &[u8] {
        &self.token_data
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = Chain<Once<Self::Item>, IntoIter<Self::Item, TOKEN_DATA_SIZE>>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.token_id).chain(self.token_data)
    }
}

impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let token_id = src.read_num_be()?;
        let token_data = src.read_array_exact::<TOKEN_DATA_SIZE>()?;
        Ok(Self {
            token_id,
            token_data,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: Status,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.status.into())
    }
}

impl Readable for Response {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let status: u8 = src.read_num_be()?;
        Ok(Self {
            status: status.try_into()?,
        })
    }
}
