use crate::counter::Counter;
use crate::frame::Parameters;
use std::io::Read;
use std::iter::{once, Once};

pub const ID: u16 = 0x00F2;

/// This call is fired when a counter exceeds its threshold.
#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    typ: Counter,
}

impl Response {
    #[must_use]
    pub const fn new(typ: Counter) -> Self {
        Self { typ }
    }

    #[must_use]
    pub const fn typ(&self) -> Counter {
        self.typ
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.typ.into())
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [typ] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            typ: Counter::try_from(typ)?,
        })
    }
}
