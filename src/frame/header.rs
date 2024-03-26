mod control;

pub use control::{CallbackType, Control, FrameFormatVersion, HighByte, LowByte, SleepMode};
use le_stream::derive::{FromLeBytes, ToLeBytes};
use le_stream::{FromLeBytes, ToLeBytes};
use std::fmt::Debug;

#[derive(Copy, Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Header<T>
where
    T: Copy + Debug + Eq + FromLeBytes + ToLeBytes,
{
    sequence: u8,
    control: T,
    id: T,
}

impl<T> Header<T>
where
    T: Copy + Debug + Eq + Into<Control> + FromLeBytes + ToLeBytes,
{
    #[must_use]
    pub const fn new(sequence: u8, control: T, id: T) -> Self {
        Self {
            sequence,
            control,
            id,
        }
    }

    #[must_use]
    pub fn control(&self) -> Control {
        self.control.into()
    }

    #[must_use]
    pub const fn id(&self) -> T {
        self.id
    }
}
