/// A trait to parse any `EZSP` frame given its frame ID from a little-endian byte stream.
pub trait Parse: Sized {
    /// Parse a, `EZSP` frame from a little-endian byte stream.
    fn parse<T>(id: u16, stream: T) -> le_stream::Result<Self>
    where
        T: Iterator<Item = u8>;
}
