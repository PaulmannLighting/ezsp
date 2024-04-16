mod control;

pub use control::{CallbackType, Control, FrameFormatVersion, HighByte, LowByte, SleepMode};
use le_stream::{FromLeBytes, ToLeBytes};
use std::fmt::Debug;
use std::iter::Chain;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Header<T>
where
    T: Copy + Debug + Eq,
{
    sequence: u8,
    control: T,
    id: T,
}

impl<T> Header<T>
where
    T: Copy + Debug + Eq + From<Control> + Into<Control>,
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

impl<T> FromLeBytes for Header<T>
where
    T: Copy + Debug + Eq + From<Control> + Into<Control> + FromLeBytes,
{
    fn from_le_bytes<I>(bytes: &mut I) -> le_stream::Result<Self>
    where
        I: Iterator<Item = u8>,
    {
        let sequence = <u8 as FromLeBytes>::from_le_bytes(bytes)?;
        let control = T::from_le_bytes(bytes)?;
        let id = T::from_le_bytes(bytes)?;
        Ok(Self::new(sequence, control, id))
    }
}

impl<T> ToLeBytes for Header<T>
where
    T: Copy + Debug + Eq + ToLeBytes,
{
    type Iter =
        Chain<<u8 as ToLeBytes>::Iter, Chain<<T as ToLeBytes>::Iter, <T as ToLeBytes>::Iter>>;

    fn to_le_bytes(self) -> Self::Iter {
        ToLeBytes::to_le_bytes(self.sequence)
            .chain(self.control.to_le_bytes().chain(self.id.to_le_bytes()))
    }
}
