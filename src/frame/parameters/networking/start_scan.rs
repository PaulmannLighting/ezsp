use le_stream::derive::{FromLeStream, ToLeStream};
use siliconlabs::Status;

use crate::ezsp::network::scan::Type;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x001A;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    scan_type: u8,
    channel_mask: u32,
    duration: u8,
}

impl Command {
    #[must_use]
    pub fn new(scan_type: Type, channel_mask: u32, duration: u8) -> Self {
        Self {
            scan_type: scan_type.into(),
            channel_mask,
            duration,
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub(crate) struct Response {
    status: VariableLengthU32,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(u32::from(self.status)).resolve()
    }
}

#[allow(clippy::struct_field_names)]
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
struct VariableLengthU32 {
    byte_1: u8,
    byte_2: Option<u8>,
    byte_3: Option<u8>,
    byte_4: Option<u8>,
}

impl From<VariableLengthU32> for u32 {
    fn from(status: VariableLengthU32) -> Self {
        let mut result = Self::from(status.byte_1);

        if let Some(byte) = status.byte_2 {
            result |= Self::from(byte) << 8;
        }
        if let Some(byte) = status.byte_3 {
            result |= Self::from(byte) << 16;
        }
        if let Some(byte) = status.byte_4 {
            result |= Self::from(byte) << 24;
        }

        result
    }
}
