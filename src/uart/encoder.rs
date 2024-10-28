use std::io::ErrorKind;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use ashv2::{Payload, MAX_PAYLOAD_SIZE};
use le_stream::ToLeStream;
use tokio::sync::mpsc::Sender;

use crate::frame::Header;
use crate::Error;
use crate::{MAX_HEADER_SIZE, MAX_PARAMETER_SIZE};

/// Codec to encode frames to bytes and decode bytes into frames.
///
/// This can be used with `tokio::encoder::Framed` to encode and decode frames.
#[derive(Debug)]
pub struct Encoder {
    sender: Sender<Payload>,
    legacy: Arc<AtomicBool>,
    header: heapless::Vec<u8, MAX_HEADER_SIZE>,
    parameters: heapless::Vec<u8, MAX_PARAMETER_SIZE>,
}

impl Encoder {
    pub fn new(sender: Sender<Payload>, legacy: Arc<AtomicBool>) -> Self {
        Self {
            sender,
            legacy,
            header: heapless::Vec::new(),
            parameters: heapless::Vec::new(),
        }
    }

    pub async fn send<T>(&mut self, header: Header, parameters: T) -> Result<(), Error>
    where
        T: ToLeStream,
    {
        self.header.clear();
        self.parameters.clear();

        match header {
            Header::Legacy(header) => self.header.extend(header.to_le_stream()),
            Header::Extended(header) => self.header.extend(header.to_le_stream()),
        };

        self.parameters.extend(parameters.to_le_stream());

        if self.parameters.is_empty() {
            // If there are no parameters to send, e.g. on `nop`, a call to `.chunks()`
            // would yield an empty iterator, resulting in us not even sending the header.
            let mut payload = heapless::Vec::new();
            payload.extend_from_slice(&self.header).map_err(|()| {
                std::io::Error::new(ErrorKind::OutOfMemory, "Payload buffer overflow")
            })?;
            self.sender.send(payload).await.map_err(|_| {
                std::io::Error::new(ErrorKind::BrokenPipe, "Failed to send payload")
            })?;
            return Ok(());
        }

        for chunk in self
            .parameters
            .chunks(MAX_PAYLOAD_SIZE.saturating_sub(self.header.len()))
        {
            let mut payload = heapless::Vec::new();
            payload.extend_from_slice(&self.header).map_err(|()| {
                std::io::Error::new(ErrorKind::OutOfMemory, "Payload buffer overflow")
            })?;
            payload.extend_from_slice(chunk).map_err(|()| {
                std::io::Error::new(ErrorKind::OutOfMemory, "Payload buffer overflow")
            })?;
            self.sender.send(payload).await.map_err(|_| {
                std::io::Error::new(ErrorKind::BrokenPipe, "Failed to send payload")
            })?;
        }

        Ok(())
    }
}
