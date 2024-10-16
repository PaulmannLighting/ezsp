use ashv2::{HexSlice, MAX_PAYLOAD_SIZE};
use log::info;

use le_stream::{FromLeStream, ToLeStream};
use std::fmt::Debug;
use tokio_util::bytes::BytesMut;
use tokio_util::codec::Decoder;

use crate::constants::EZSP_MAX_FRAME_SIZE;
use crate::error::{Decode, Error};
use crate::frame::{Extended, Handler, Header};

/// Codec to encode frames to bytes and decode bytes into frames.
///
/// This can be used with `tokio::codec::Framed` to encode and decode frames.
#[derive(Debug)]
pub struct Codec {
    id: Option<u16>,
    buffer: heapless::Vec<u8, EZSP_MAX_FRAME_SIZE>,
}

impl Codec {
    fn parse(&mut self) -> Result<Option<Handler>, Decode> {
        let Some(id) = self.id.take() else {
            return Ok(None);
        };

        info!(
            "Parsing parameters from: {:#04X}",
            HexSlice::new(&self.buffer)
        );

        let result = Handler::parse_from_le_stream(id, self.buffer.iter().copied());
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
    type Item = Handler;
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
                    return Ok(self.parse()?);
                }
            } else {
                self.id.replace(header.id());
            }

            self.buffer.extend(stream);
        }

        src.clear();
        Ok(self.parse()?)
    }
}
