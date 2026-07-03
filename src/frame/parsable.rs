use le_stream::FromLeStream;

pub use self::warn_excess_bytes::WarnExcessBytes;
use crate::error::Decode;
use crate::frame::Parameter;

mod warn_excess_bytes;

/// A trait for parsing parameters from a little-endian stream given their frame ID.
pub trait Parsable: Sized {
    /// Parse a parameter from a little-endian stream given its frame ID.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::error::Error) if the parsing of the parameter failed.
    fn parse_from_le_stream<T>(id: u16, stream: T) -> Result<Self, Decode>
    where
        T: Iterator<Item = u8>;
}

impl<T> Parsable for T
where
    T: Parameter + FromLeStream,
{
    fn parse_from_le_stream<S>(id: u16, mut stream: S) -> Result<Self, Decode>
    where
        S: Iterator<Item = u8>,
    {
        if Self::ID != id {
            return Err(Decode::FrameIdMismatch {
                expected: Self::ID,
                found: id,
            });
        }

        Self::from_le_stream_exact(&mut stream).warn_excess_bytes(stream)
    }
}
