use crate::ember::counter::Type;
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::iter::{once, Once};

pub const ID: u16 = 0x00F2;

/// This call is fired when a counter exceeds its threshold.
#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    typ: Type,
}

impl Response {
    #[must_use]
    pub const fn new(typ: Type) -> Self {
        Self { typ }
    }

    #[must_use]
    pub const fn typ(&self) -> Type {
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

impl Readable for Response {
    fn try_read<R>(src: &mut R) -> Result<Self, crate::Error>
    where
        R: Read,
    {
        let typ: u8 = src.read_num_le()?;
        Ok(Self {
            typ: typ.try_into()?,
        })
    }
}
