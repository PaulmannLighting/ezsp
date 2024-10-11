use crate::frame::{Control, Frame, Header, Parameter};
use crate::transport::parse_response::parse_response;
use crate::transport::Transport;
use crate::Error;
use ashv2::AshFramed;
use futures::SinkExt;
use le_stream::{FromLeStream, ToLeStream};
use log::debug;
use std::fmt::Debug;
use tokio::io::ReadBuf;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

/// ASHv2 transport layer implementation.
#[derive(Debug)]
pub struct Ashv2<const BUF_SIZE: usize> {
    framed: Framed<AshFramed<BUF_SIZE>, RawCodec>,
    sequence: u8,
    control: Control,
    buffer: Vec<u8>,
}

impl<const BUF_SIZE: usize> Ashv2<BUF_SIZE> {
    /// Creates an ASHv2 host.
    #[must_use]
    pub fn new(ash: AshFramed<BUF_SIZE>, control: Control) -> Self {
        Self {
            framed: Framed::new(ash, RawCodec),
            sequence: 0,
            control,
            buffer: Vec::new(),
        }
    }
}

impl<const BUF_SIZE: usize> Transport for Ashv2<BUF_SIZE> {
    fn next_header<P>(&mut self) -> Header<P::Id>
    where
        P: Parameter,
    {
        let sequence = self.sequence;
        let header = Header::new(sequence, self.control.into(), P::ID);
        debug!("Header: {header:?}");
        self.sequence = sequence.wrapping_add(1);
        header
    }

    async fn communicate<C, R>(&mut self, command: C) -> Result<R, Error>
    where
        C: Parameter + ToLeStream + Send,
        R: Clone + Debug + Send + Parameter + FromLeStream,
    {
        self.buffer.clear();
        let header = self.next_header::<R>();
        self.buffer.extend(header.to_le_stream());
        self.buffer.extend(command.to_le_stream());
        debug!("Sending payload: {:#04X?}", self.buffer);
        self.framed.send(self.buffer.as_slice().into()).await?;

        if let Some(response) = self.framed.next().await {
            let response = response?;
            debug!("Received payload: {:#04X?}", response);
            let frame = parse_response::<Frame<R>>(response.as_ref())?;
            return Ok(frame.parameters());
        }

        Err(Error::Custom("foo".into()))
    }
}
use tokio_util::bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

/// An example decoder.
#[derive(Debug)]
pub struct RawCodec;

impl Decoder for RawCodec {
    type Item = Box<[u8]>;
    type Error = std::io::Error;

    fn decode(&mut self, buffer: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if buffer.len() >= 4 {
            Ok(Some(buffer.split().as_ref().into()))
        } else {
            Ok(None)
        }
    }
}

impl Encoder<Box<[u8]>> for RawCodec {
    type Error = std::io::Error;

    fn encode(&mut self, item: Box<[u8]>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.extend_from_slice(&item);
        Ok(())
    }
}
