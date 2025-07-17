use std::io::{self, ErrorKind};

use ashv2::{MAX_PAYLOAD_SIZE, Payload};
use le_stream::ToLeStream;
use tokio::sync::mpsc::Sender;

use crate::frame::Header;
use crate::{Error, MAX_HEADER_SIZE, MAX_PARAMETER_SIZE};

/// Encode `EZSP` frames into `ASHv2` frames.
#[derive(Debug)]
pub struct Encoder {
    sender: Sender<Payload>,
    header: heapless::Vec<u8, MAX_HEADER_SIZE>,
    parameters: heapless::Vec<u8, MAX_PARAMETER_SIZE>,
}

impl Encoder {
    /// Create a new `Encoder`.
    pub const fn new(sender: Sender<Payload>) -> Self {
        Self {
            sender,
            header: heapless::Vec::new(),
            parameters: heapless::Vec::new(),
        }
    }

    /// Encode an `EZSP` header and parameters into a `ASHv2` frames.
    pub async fn send<T>(&mut self, header: Header, parameters: T) -> Result<(), Error>
    where
        T: ToLeStream + Send,
    {
        self.header.clear();
        self.parameters.clear();

        match header {
            Header::Legacy(header) => self.header.extend(header.to_le_stream()),
            Header::Extended(header) => self.header.extend(header.to_le_stream()),
        }

        self.parameters.extend(parameters.to_le_stream());

        if self.parameters.is_empty() {
            // If there are no parameters to send, e.g. on `nop`, a call to `.chunks()`
            // would yield an empty iterator, resulting in us not even sending the header.
            self.send_chunk(None).await?;
        }

        for chunk in self
            .parameters
            .chunks(MAX_PAYLOAD_SIZE.saturating_sub(self.header.len()))
        {
            self.send_chunk(Some(chunk)).await?;
        }

        Ok(())
    }

    async fn send_chunk(&self, chunk: Option<&[u8]>) -> io::Result<()> {
        let mut payload = heapless::Vec::new();
        payload
            .extend_from_slice(&self.header)
            .map_err(|()| buffer_overflow())?;

        if let Some(chunk) = chunk {
            payload
                .extend_from_slice(chunk)
                .map_err(|()| buffer_overflow())?;
        }

        self.sender
            .send(payload)
            .await
            .map_err(|_| failed_to_send_payload())
    }
}

fn buffer_overflow() -> io::Error {
    io::Error::new(ErrorKind::OutOfMemory, "Payload buffer overflow")
}

fn failed_to_send_payload() -> io::Error {
    io::Error::new(ErrorKind::BrokenPipe, "Failed to send payload")
}
