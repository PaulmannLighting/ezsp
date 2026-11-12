use std::fmt::Debug;
use std::iter::once;

use le_stream::Error;
use log::{debug, warn};

use crate::error::Decode;

pub trait WarnExcessBytes<T> {
    fn warn_excess_bytes<I>(self, stream: I) -> Result<T, Decode>
    where
        I: Iterator<Item = u8>;
}

impl<T> WarnExcessBytes<T> for Result<T, Error<T>>
where
    T: Debug,
{
    fn warn_excess_bytes<I>(self, stream: I) -> Result<T, Decode>
    where
        I: Iterator<Item = u8>,
    {
        match self {
            Ok(v) => Ok(v),
            Err(Error::StreamNotExhausted {
                next_byte,
                instance,
            }) => {
                warn!("Stream not exhausted after parsing: {instance:?}");
                // Trailing bytes can be unparsed secret material on a mismatched firmware.
                let count = once(next_byte).chain(stream).count();
                debug!("Discarded {count} excess bytes");
                Ok(instance)
            }
            Err(Error::UnexpectedEndOfStream) => Err(Decode::TooFewBytes),
        }
    }
}
