use ashv2::{HexSlice, MAX_PAYLOAD_SIZE};
use log::info;

use le_stream::{FromLeStream, ToLeStream};
use std::fmt::Debug;
use tokio_util::bytes::BytesMut;
use tokio_util::codec::Decoder;

use crate::constants::EZSP_MAX_FRAME_SIZE;
use crate::error::Error;
use crate::frame::{
    parameters::networking::handler::{network_found, scan_complete},
    parameters::utilities::invalid_command,
    Extended, Header, Parameter,
};

/// Codec to encode frames to bytes and decode bytes into frames.
///
/// This can be used with `tokio::codec::Framed` to encode and decode frames.
#[derive(Debug)]
pub struct Codec {
    id: Option<u16>,
    buffer: heapless::Vec<u8, EZSP_MAX_FRAME_SIZE>,
}

impl Codec {
    fn parse(&mut self) -> Result<Option<Callback>, Error> {
        let Some(id) = self.id.take() else {
            return Ok(None);
        };

        info!(
            "Parsing parameters from: {:#04X}",
            HexSlice::new(&self.buffer)
        );

        let result = Callback::parse(id, &self.buffer);
        self.buffer.clear();
        result.map(Some)
    }
}

impl Default for Codec {
    fn default() -> Self {
        Self {
            id: None,
            buffer: heapless::Vec::new(),
        }
    }
}

impl Decoder for Codec {
    type Item = Callback;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        self.buffer.clear();

        for (index, chunk) in src.chunks(MAX_PAYLOAD_SIZE).enumerate() {
            let mut stream = chunk.iter().copied();

            let Some(header) = Extended::from_le_stream(&mut stream) else {
                return Ok(None);
            };

            if let Some(id) = self.id {
                if <Extended as Header<u16>>::id(header) != id {
                    let _ = src.split_at(index * header.to_le_stream().count());
                    return self.parse();
                }
            } else {
                self.id.replace(header.id());
            }

            self.buffer.extend(stream);
        }

        src.clear();
        self.parse()
    }
}

#[derive(Debug)]
pub enum Callback {
    NetworkFound(network_found::Handler),
    ScanComplete(scan_complete::Handler),
}

impl Callback {
    pub fn parse(id: u16, buffer: &[u8]) -> Result<Self, Error> {
        match id {
            invalid_command::Response::ID => {
                info!("Found invalid command response");
                Err(Error::InvalidCommand(
                    invalid_command::Response::from_le_slice(buffer)?,
                ))
            }
            network_found::Handler::ID => {
                info!("Found network found callback");
                Ok(Self::NetworkFound(network_found::Handler::from_le_slice(
                    buffer,
                )?))
            }
            scan_complete::Handler::ID => {
                info!("Found scan complete callback");
                Ok(Self::ScanComplete(scan_complete::Handler::from_le_slice(
                    buffer,
                )?))
            }
            id => Err(Error::Custom(format!("Invalid callback ID: {id}"))),
        }
    }
}
