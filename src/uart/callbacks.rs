mod stream;

use ashv2::Payload;
use stream::Stream;

/// Iterator extension to parse `EZSP` callback ("handler") frames from `ASHv2` data frames.
pub trait Callbacks: Iterator<Item = Payload> + Sized {
    /// Returns an iterator over `EZSP` callback ("handler") frames.
    fn callbacks(self) -> Stream<Self>;
}

impl<T> Callbacks for T
where
    T: Iterator<Item = Payload>,
{
    fn callbacks(self) -> Stream<Self> {
        Stream::new(self)
    }
}
