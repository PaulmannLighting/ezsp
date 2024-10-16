mod callback_stream;

use ashv2::Payload;
use callback_stream::CallbackStream;

/// Iterator extension to parse `EZSP` callback ("handler") frames from `ASHv2` data frames.
pub trait Callbacks: Iterator<Item = Payload> + Sized {
    /// Returns an iterator over `EZSP` callback ("handler") frames.
    fn callbacks(self) -> CallbackStream<Self>;
}

impl<T> Callbacks for T
where
    T: Iterator<Item = Payload>,
{
    fn callbacks(self) -> CallbackStream<Self> {
        CallbackStream::new(self)
    }
}
