use crate::ember_status::EmberStatus;
use crate::frame::Parameters;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{empty, Empty};

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

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(_: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Ok(Self {})
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: EmberStatus,
    manufacturer_id: u16,
    version_number: u16,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, manufacturer_id: u16, version_number: u16) -> Self {
        Self {
            status,
            manufacturer_id,
            version_number,
        }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
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
    type IntoIter = IntoIter<Self::Item, 5>;

    fn into_iter(self) -> Self::IntoIter {
        let [manufacturer_id_low, manufacturer_id_high] = self.manufacturer_id.to_be_bytes();
        let [version_number_low, version_number_high] = self.version_number.to_be_bytes();
        [
            self.status.into(),
            manufacturer_id_low,
            manufacturer_id_high,
            version_number_low,
            version_number_high,
        ]
        .into_iter()
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @
        [status, manufacturer_id_low, manufacturer_id_high, version_number_low, version_number_high] =
            [0; 5];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            status: EmberStatus::try_from(status)?,
            manufacturer_id: u16::from_be_bytes([manufacturer_id_low, manufacturer_id_high]),
            version_number: u16::from_be_bytes([version_number_low, version_number_high]),
        })
    }
}
