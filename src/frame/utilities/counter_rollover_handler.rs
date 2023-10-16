use crate::counter::Counter;
use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use std::io::Read;

const ID: u16 = 0x00F2;

/// This call is fired when a counter exceeds its threshold.
#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    typ: Counter,
}

impl Response {
    #[must_use]
    pub const fn new(sequence: u8, control: Control, typ: Counter) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            typ,
        }
    }

    #[must_use]
    pub const fn typ(&self) -> Counter {
        self.typ
    }
}

impl Frame<ID> for Response {
    type Parameters = [u8; 1];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([self.typ.into()])
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [typ]: [u8; 1] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            header,
            typ: Counter::try_from(typ)?,
        })
    }
}
