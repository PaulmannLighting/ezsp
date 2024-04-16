mod header;
pub mod parameters;

pub use header::{CallbackType, Control, FrameFormatVersion, Header, HighByte, LowByte, SleepMode};
use le_stream::{FromLeBytes, ToLeBytes};
pub use parameters::Parameter;
use std::fmt::Debug;
use std::iter::Chain;

#[derive(Debug)]
pub struct Frame<P>
where
    P: Parameter,
{
    header: Header<P::Id>,
    parameters: P,
}

impl<P> Frame<P>
where
    P: Parameter,
{
    #[must_use]
    pub const fn new(header: Header<P::Id>, parameters: P) -> Self {
        Self { header, parameters }
    }
    pub fn parameters(self) -> P {
        self.parameters
    }
}

impl<P> FromLeBytes for Frame<P>
where
    P: Parameter + FromLeBytes,
    <P as Parameter>::Id: FromLeBytes,
{
    fn from_le_bytes<T>(bytes: &mut T) -> le_stream::Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        let header = Header::from_le_bytes(bytes)?;
        let parameters = P::from_le_bytes(bytes)?;
        Ok(Self::new(header, parameters))
    }
}

impl<P> ToLeBytes for Frame<P>
where
    P: Parameter + ToLeBytes,
    <P as Parameter>::Id: ToLeBytes,
{
    type Iter = Chain<<Header<P::Id> as ToLeBytes>::Iter, <P as ToLeBytes>::Iter>;

    fn to_le_bytes(self) -> Self::Iter {
        self.header
            .to_le_bytes()
            .chain(self.parameters.to_le_bytes())
    }
}
