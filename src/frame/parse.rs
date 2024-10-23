use crate::error::Decode;

/// A trait to parse any `EZSP` frame given its frame ID from a little-endian byte stream.
pub trait Parse: Sized {
    /// Parse an `EZSP` frame from a little-endian byte stream.
    fn parse<T>(id: u16, stream: T) -> Result<Self, Decode>
    where
        T: Iterator<Item = u8>;
}
