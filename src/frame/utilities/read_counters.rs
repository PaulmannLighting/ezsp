use crate::counter::Counter;
use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use anyhow::anyhow;
use never::Never;
use std::io::Read;

const ID: u16 = 0x00F1;
const TYPE_COUNT: usize = Counter::TypeCount as usize;

/// Retrieves Ember counters.
///
/// See the [`Counter`] enumeration for the counter types.
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
    values: [u16; TYPE_COUNT],
}

impl Response {
    #[must_use]
    pub const fn new(sequence: u8, control: Control, values: [u16; TYPE_COUNT]) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            values,
        }
    }

    #[must_use]
    pub const fn values(&self) -> &[u16] {
        &self.values
    }
}

impl Frame<ID> for Response {
    type Parameters = Vec<u8>;

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        let mut parameters = Vec::with_capacity(2 * (Counter::TypeCount as usize));
        self.values
            .iter()
            .for_each(|value| parameters.extend_from_slice(&value.to_be_bytes()));
        Some(parameters)
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer = [0; TYPE_COUNT * 2];
        src.read_exact(&mut buffer)?;
        let values: Vec<u16> = buffer
            .chunks_exact(2)
            .filter_map(|chunk| {
                if chunk.len() == 2 {
                    Some(u16::from_be_bytes([chunk[0], chunk[1]]))
                } else {
                    None
                }
            })
            .collect();
        Ok(Self {
            header,
            values: values
                .try_into()
                .map_err(|_| anyhow!("values size != {TYPE_COUNT}"))?,
        })
    }
}
