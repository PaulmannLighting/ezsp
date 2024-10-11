use super::{ExtendedFrameControl, ValidControl};
use le_stream::derive::{FromLeStream, ToLeStream};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct Extended<T>
where
    T: ValidControl,
{
    pub(crate) low_byte: T,
    pub(crate) high_byte: ExtendedFrameControl,
}
