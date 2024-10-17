use crate::error::Decode;
use crate::frame::Parameter;
use le_stream::FromLeStream;

pub trait Parsable: Sized {
    fn parse_from_le_stream<T>(id: u16, stream: T) -> Result<Self, Decode>
    where
        T: Iterator<Item = u8>;
}

impl<T> Parsable for T
where
    T: Parameter + FromLeStream,
{
    fn parse_from_le_stream<S>(id: u16, stream: S) -> Result<Self, Decode>
    where
        S: Iterator<Item = u8>,
    {
        let Some(my_id) = Self::ID else {
            return Err(Decode::MissingId);
        };

        if my_id.into() != id {
            return Err(Decode::FrameIdMismatch {
                expected: my_id.into(),
                found: id,
            });
        }

        Ok(Self::from_le_stream_exact(stream)?)
    }
}
