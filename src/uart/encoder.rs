use core::fmt::Debug;
use std::io;

use ashv2::{Handle, Payload};
use heapless::LenType;
use le_stream::ToLeStream;
use log::trace;

use crate::Error;
use crate::frame::Header;

/// Encodes EZSP headers and parameters into `ASHv2` DATA payloads.
#[derive(Debug)]
pub struct Encoder {
    ash_v2: Handle,
}

impl Encoder {
    /// Create a new `Encoder`.
    pub const fn new(ash_v2: Handle) -> Self {
        Self { ash_v2 }
    }

    /// Encode an EZSP header and parameters into one or more `ASHv2` payloads.
    ///
    /// `ASHv2` DATA fields have a bounded payload size. Each emitted payload
    /// repeats the EZSP header and carries the next parameter chunk.
    pub async fn send<T>(&self, header: Header, parameters: T) -> Result<(), Error>
    where
        T: Debug + ToLeStream,
    {
        trace!("Sending EZSP frame: Header: {header:#04X?}, parameters: {parameters:?}");
        let mut payload = Payload::new();

        match header {
            Header::Legacy(header) => payload.try_extend(header.to_le_stream())?,
            Header::Extended(header) => payload.try_extend(header.to_le_stream())?,
        }

        payload.try_extend(parameters.to_le_stream())?;
        trace!("Sending EZSP frame (bytes): {payload:#04X?}");
        Ok(self.ash_v2.send(payload).await?)
    }
}

trait TryExtend<T> {
    fn try_extend<U>(&mut self, iter: U) -> io::Result<()>
    where
        U: IntoIterator<Item = T>;
}

impl<const SIZE: usize, T, LenT> TryExtend<T> for heapless::Vec<T, SIZE, LenT>
where
    LenT: LenType,
{
    fn try_extend<I>(&mut self, iter: I) -> io::Result<()>
    where
        I: IntoIterator<Item = T>,
    {
        for elem in iter {
            self.push(elem)
                .map_err(|_| io::Error::from(io::ErrorKind::OutOfMemory))?;
        }

        Ok(())
    }
}
