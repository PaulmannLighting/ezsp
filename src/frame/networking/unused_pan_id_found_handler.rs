use crate::frame::Parameters;
use rw_exact_ext::ReadExactExt;
use std::array::IntoIter;
use std::io::Read;
use std::iter::Chain;

pub const ID: u16 = 0x001C;

/// This function returns an unused panID and channel pair
/// found via the find unused panId scan procedure.
#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    pan_id: u16,
    channel: u8,
}

impl Response {
    #[must_use]
    pub const fn new(pan_id: u16, channel: u8) -> Self {
        Self { pan_id, channel }
    }

    #[must_use]
    pub const fn pan_id(&self) -> u16 {
        self.pan_id
    }

    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = Chain<IntoIter<u8, 2>, IntoIter<u8, 1>>;

    fn into_iter(self) -> Self::IntoIter {
        self.pan_id
            .to_be_bytes()
            .into_iter()
            .chain(self.channel.to_be_bytes())
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let pan_id = src.read_num_be()?;
        let channel = src.read_num_be()?;
        Ok(Self { pan_id, channel })
    }
}
