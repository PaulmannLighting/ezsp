use crate::ember::Status;
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{empty, once, Chain, Empty, Once};

pub const ID: u16 = 0x0013;

/// Allows the HOST to know whether the NCP is running the XNCP library.
///
/// If so, the response contains also the manufacturer ID and the
/// version number of the XNCP application that is running on the NCP.
#[derive(Debug, Eq, PartialEq)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = Empty<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        empty()
    }
}

impl Readable for Command {
    fn try_read<R>(_: &mut R) -> Result<Self, crate::Error>
    where
        R: Read,
    {
        Ok(Self {})
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: Status,
    manufacturer_id: u16,
    version_number: u16,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status, manufacturer_id: u16, version_number: u16) -> Self {
        Self {
            status,
            manufacturer_id,
            version_number,
        }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }

    #[must_use]
    pub const fn manufacturer_id(&self) -> u16 {
        self.manufacturer_id
    }

    #[must_use]
    pub const fn version_number(&self) -> u16 {
        self.version_number
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter =
        Chain<Chain<Once<Self::Item>, IntoIter<Self::Item, 2>>, IntoIter<Self::Item, 2>>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.status.into())
            .chain(self.manufacturer_id.to_le_bytes())
            .chain(self.version_number.to_le_bytes())
    }
}

impl Readable for Response {
    fn try_read<R>(src: &mut R) -> Result<Self, crate::Error>
    where
        R: Read,
    {
        let status: u8 = src.read_num_le()?;
        let manufacturer_id = src.read_num_le()?;
        let version_number = src.read_num_le()?;
        Ok(Self {
            status: status.try_into()?,
            manufacturer_id,
            version_number,
        })
    }
}
