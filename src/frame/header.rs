mod control;

pub use control::Control;
use le_stream::derive::{FromLeBytes, ToLeBytes};
use le_stream::{FromLeBytes, ToLeBytes};
use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Header<C, I>
where
    C: Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
    I: Copy + Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
{
    sequence: u8,
    control: C,
    id: I,
}

impl<C, I> Header<C, I>
where
    C: Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
    I: Copy + Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
{
    #[must_use]
    pub const fn new(sequence: u8, control: C, id: I) -> Self {
        Self {
            sequence,
            control,
            id,
        }
    }

    #[must_use]
    pub const fn control(&self) -> &C {
        &self.control
    }

    #[must_use]
    pub const fn id(&self) -> I {
        self.id
    }
}
