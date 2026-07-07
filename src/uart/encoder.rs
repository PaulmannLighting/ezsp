use core::fmt::Debug;
use std::io;

use ashv2::{Handle, MAX_PAYLOAD_SIZE, Payload};
use le_stream::ToLeStream;
use log::trace;

use crate::frame::Header;
use crate::{Error, MAX_HEADER_SIZE, MAX_PARAMETER_SIZE};

/// Encodes EZSP headers and parameters into `ASHv2` DATA payloads.
#[derive(Debug)]
pub struct Encoder {
    ash_v2: Handle,
    header: heapless::Vec<u8, MAX_HEADER_SIZE>,
    parameters: heapless::Vec<u8, MAX_PARAMETER_SIZE>,
}

impl Encoder {
    /// Create a new `Encoder`.
    pub const fn new(ash_v2: Handle) -> Self {
        Self {
            ash_v2,
            header: heapless::Vec::new(),
            parameters: heapless::Vec::new(),
        }
    }

    /// Encode an EZSP header and parameters into one or more `ASHv2` payloads.
    ///
    /// `ASHv2` DATA fields have a bounded payload size. Each emitted payload
    /// repeats the EZSP header and carries the next parameter chunk.
    pub async fn send<P>(&mut self, header: Header, parameters: P) -> Result<(), Error>
    where
        P: Debug + ToLeStream,
    {
        self.header.clear();
        self.parameters.clear();

        match header {
            Header::Legacy(header) => self.header.extend(header.to_le_stream()),
            Header::Extended(header) => self.header.extend(header.to_le_stream()),
        }
        trace!("Sending header: {:#04X?}", self.header);

        trace!("Sending parameters (type): {parameters:?}");
        self.parameters.extend(parameters.to_le_stream());
        trace!("Sending parameters (bytes): {:#04X?}", self.parameters);

        if self.parameters.is_empty() {
            // If there are no parameters to send, e.g. on `nop`, a call to `.chunks()`
            // would yield an empty iterator, resulting in us not even sending the header.
            self.send_chunk(&[]).await?;
        }

        trace!("Sending chunks of {MAX_PAYLOAD_SIZE} bytes.");
        for chunk in self
            .parameters
            .chunks(MAX_PAYLOAD_SIZE.saturating_sub(self.header.len()))
        {
            self.send_chunk(chunk).await?;
        }

        Ok(())
    }

    async fn send_chunk(&self, chunk: &[u8]) -> io::Result<()> {
        let mut payload = Payload::new();
        payload
            .extend_from_slice(&self.header)
            .map_err(io::Error::other)?;
        payload.extend_from_slice(chunk).map_err(io::Error::other)?;
        trace!("Sending chunk: {payload:#04X?}");
        self.ash_v2.send(payload).await
    }
}
