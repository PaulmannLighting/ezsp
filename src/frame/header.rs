mod control;

pub use control::ValidControl;
pub use control::{
    CallbackType, Command, Control, Extended, FrameFormatVersion, Response, SleepMode,
};
use le_stream::derive::{FromLeStream, ToLeStream};
use std::fmt::Debug;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct Header<T>
where
    T: ValidControl,
{
    sequence: u8,
    control: Control<T>,
    id: T::Size,
}

impl<T> Header<T>
where
    T: ValidControl,
{
    #[must_use]
    pub const fn new(sequence: u8, control: Control<T>, id: T::Size) -> Self {
        Self {
            sequence,
            control,
            id,
        }
    }

    #[must_use]
    pub const fn sequence(&self) -> u8 {
        self.sequence
    }

    #[must_use]
    pub fn control(&self) -> Control<T> {
        self.control
    }

    #[must_use]
    pub const fn id(&self) -> T::Size {
        self.id
    }
}
