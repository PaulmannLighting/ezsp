use crate::frame::Parameters;
use std::array::IntoIter;
use std::io::Read;
use std::iter::Chain;

pub const ID: u16 = 0x0048;

/// Reports the result of an energy scan for a single channel.
///
/// The scan is not complete until the scanCompleteHandler callback is called.
#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    channel: u8,
    max_rssi_value: i8,
}

impl Response {
    #[must_use]
    pub const fn new(channel: u8, max_rssi_value: i8) -> Self {
        Self {
            channel,
            max_rssi_value,
        }
    }

    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }

    #[must_use]
    pub const fn max_rssi_value(&self) -> i8 {
        self.max_rssi_value
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = Chain<IntoIter<u8, 1>, IntoIter<u8, 1>>;

    fn into_iter(self) -> Self::IntoIter {
        self.channel
            .to_be_bytes()
            .into_iter()
            .chain(self.max_rssi_value.to_be_bytes())
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [channel, max_rssi_value @ ..] = [0; 2];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            channel,
            max_rssi_value: i8::from_be_bytes(max_rssi_value),
        })
    }
}
