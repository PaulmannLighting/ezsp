mod control;

pub use control::{CallbackType, Control, FrameFormatVersion, HighByte, LowByte, SleepMode};
use le_stream::{FromLeStream, ToLeStream};
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
    T: Copy + Debug + Eq + From<Control> + Into<Control> + Into<u16>,
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

impl<T> FromLeStream for Header<T>
where
    T: Copy + Debug + Eq + From<Control> + Into<Control> + Into<u16> + FromLeStream,
{
    fn from_le_stream<I>(bytes: &mut I) -> le_stream::Result<Self>
    where
        I: Iterator<Item = u8>,
    {
        let sequence = <u8 as FromLeStream>::from_le_stream(bytes)?;
        let control = T::from_le_stream(bytes)?;
        let id = T::from_le_stream(bytes)?;
        Ok(Self::new(sequence, control, id))
    }
}

impl<T> ToLeStream for Header<T>
where
    T: Copy + Debug + Eq + ToLeStream,
{
    type Iter =
        Chain<<u8 as ToLeStream>::Iter, Chain<<T as ToLeStream>::Iter, <T as ToLeStream>::Iter>>;

    fn to_le_stream(self) -> Self::Iter {
        ToLeStream::to_le_stream(self.sequence)
            .chain(self.control.to_le_stream().chain(self.id.to_le_stream()))
    }
}
