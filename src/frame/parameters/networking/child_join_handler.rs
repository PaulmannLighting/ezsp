use crate::ember::node::Type;
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{once, Chain, Once};

pub const ID: u16 = 0x0023;

/// Indicates that a child has joined or left.
#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    index: u8,
    joining: bool,
    child_id: u16,
    child_eui64: u64,
    child_type: Type,
}

impl Response {
    #[must_use]
    pub const fn new(
        index: u8,
        joining: bool,
        child_id: u16,
        child_eui64: u64,
        child_type: Type,
    ) -> Self {
        Self {
            index,
            joining,
            child_id,
            child_eui64,
            child_type,
        }
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }

    #[must_use]
    pub const fn joining(&self) -> bool {
        self.joining
    }

    #[must_use]
    pub const fn child_id(&self) -> u16 {
        self.child_id
    }

    #[must_use]
    pub const fn child_eui64(&self) -> u64 {
        self.child_eui64
    }

    #[must_use]
    pub const fn child_type(&self) -> Type {
        self.child_type
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = Chain<
        Chain<
            Chain<Chain<Once<Self::Item>, Once<Self::Item>>, IntoIter<Self::Item, 2>>,
            IntoIter<Self::Item, 8>,
        >,
        Once<Self::Item>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        once(self.index)
            .chain(once(self.joining.into()))
            .chain(self.child_id.to_le_bytes())
            .chain(self.child_eui64.to_le_bytes())
            .chain(once(self.child_type.into()))
    }
}

impl Readable for Response {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let index = src.read_num_le()?;
        let joining = src.read_bool()?;
        let child_id = src.read_num_le()?;
        let child_eui64 = src.read_num_le()?;
        let child_type: u8 = src.read_num_le()?;
        Ok(Self {
            index,
            joining,
            child_id,
            child_eui64,
            child_type: child_type.try_into()?,
        })
    }
}
