use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use never::Never;
use std::io::Read;

const ID: u16 = 0x0013;

/// Allows the HOST to know whether the NCP is running the XNCP library.
///
/// If so, the response contains also the manufacturer ID and the
/// version number of the XNCP application that is running on the NCP.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
}

impl Command {
    #[must_use]
    pub const fn new(sequence: u8, control: Control) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
        }
    }
}

impl Frame<ID> for Command {
    type Parameters = Never;

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        None
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Self::read_header(src).map(|header| Self { header })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    manufacturer_id: u16,
    version_number: u16,
}

impl Response {
    #[must_use]
    pub const fn new(
        sequence: u8,
        control: Control,
        manufacturer_id: u16,
        version_number: u16,
    ) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            manufacturer_id,
            version_number,
        }
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

impl Frame<ID> for Response {
    type Parameters = [u8; 4];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        let [manufacturer_id_low, manufacturer_id_high] = self.manufacturer_id.to_be_bytes();
        let [version_number_low, version_number_high] = self.version_number.to_be_bytes();
        Some([
            manufacturer_id_low,
            manufacturer_id_high,
            version_number_low,
            version_number_high,
        ])
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer
        @ [manufacturer_id_low, manufacturer_id_high, version_number_low, version_number_high] =
            [0; 4];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            header,
            manufacturer_id: u16::from_be_bytes([manufacturer_id_low, manufacturer_id_high]),
            version_number: u16::from_be_bytes([version_number_low, version_number_high]),
        })
    }
}
