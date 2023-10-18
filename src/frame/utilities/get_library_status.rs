use crate::frame::Parameters;
use std::array::IntoIter;
use std::io::Read;

pub const ID: u16 = 0x0001;

/// This retrieves the status of the passed library ID to determine if it is compiled into the stack.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    library_id: u8,
}

impl Command {
    #[must_use]
    pub const fn new(library_id: u8) -> Self {
        Self { library_id }
    }

    #[must_use]
    pub const fn library_id(&self) -> u8 {
        self.library_id
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 1>;

    fn into_iter(self) -> Self::IntoIter {
        self.library_id.to_be_bytes().into_iter()
    }
}

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [library_id] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self { library_id })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub const fn new(status: u8) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> u8 {
        self.status
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 1>;

    fn into_iter(self) -> Self::IntoIter {
        self.status.to_be_bytes().into_iter()
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [status] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self { status })
    }
}
