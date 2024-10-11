use super::ExtendedFrameControl;
use crate::LowByte;
use le_stream::derive::{FromLeStream, ToLeStream};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct Extended {
    pub(crate) low_byte: LowByte,
    pub(crate) high_byte: ExtendedFrameControl,
}
