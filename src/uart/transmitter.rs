use core::fmt::Debug;

use ashv2::{Handle, Payload};
use heapless::LenType;
use le_stream::ToLeStream;
use log::trace;

use crate::ezsp::Status;
use crate::frame::Commands;
use crate::{Error, Frame, Header, Transmit, ezsp};

/// Encodes EZSP headers and parameters into `ASHv2` DATA payloads.
#[derive(Debug)]
pub struct AshTx {
    ash_v2: Handle,
}

impl AshTx {
    /// Creates an EZSP transmitter around an `ASHv2` actor handle.
    pub const fn new(ash_v2: Handle) -> Self {
        Self { ash_v2 }
    }
}

impl Transmit for AshTx {
    async fn transmit(&mut self, frame: Frame<Commands>) -> Result<(), Error> {
        let (header, parameters) = frame.into();
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
    fn try_extend<U>(&mut self, iter: U) -> Result<(), Error>
    where
        U: IntoIterator<Item = T>;
}

impl<const SIZE: usize, T, LenT> TryExtend<T> for heapless::Vec<T, SIZE, LenT>
where
    LenT: LenType,
{
    fn try_extend<I>(&mut self, iter: I) -> Result<(), Error>
    where
        I: IntoIterator<Item = T>,
    {
        for elem in iter {
            self.push(elem)
                .map_err(|_| Status::Error(ezsp::Error::CommandTooLong))?;
        }

        Ok(())
    }
}
