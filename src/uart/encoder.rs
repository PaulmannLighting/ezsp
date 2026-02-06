use core::fmt::Debug;
use std::io;

use ashv2::{MAX_PAYLOAD_SIZE, Payload, Proxy};
use le_stream::ToLeStream;
use log::trace;

use crate::frame::Header;
use crate::{Error, MAX_HEADER_SIZE, MAX_PARAMETER_SIZE};

/// Encode `EZSP` frames into `ASHv2` frames.
#[derive(Debug)]
pub struct Encoder<T> {
    proxy: T,
    header: heapless::Vec<u8, MAX_HEADER_SIZE>,
    parameters: heapless::Vec<u8, MAX_PARAMETER_SIZE>,
}

impl<T> Encoder<T> {
    /// Create a new `Encoder`.
    pub const fn new(proxy: T) -> Self {
        Self {
            proxy,
            header: heapless::Vec::new(),
            parameters: heapless::Vec::new(),
        }
    }
}

impl<T> Encoder<T>
where
    T: Proxy,
{
    /// Encode an `EZSP` header and parameters into a `ASHv2` frames.
    pub async fn send<P>(&mut self, header: Header, parameters: P) -> Result<(), Error>
    where
        P: Debug + ToLeStream + Send + Sync,
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
        trace!("Sending chunk: {:#04X?}", payload);
        self.proxy
            .send(payload)
            .await
            .map_err(io::Error::other)?
            .await
            .map_err(io::Error::other)?
    }
}
