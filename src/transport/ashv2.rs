mod codec;

use crate::frame::{Control, Frame, Header, Parameter, ValidControl};
use crate::transport::parse_response::parse_response;
use crate::transport::Transport;
use crate::Error;
use ashv2::AshFramed;
use codec::RawCodec;
use futures::SinkExt;
use le_stream::{FromLeStream, ToLeStream};
use log::debug;
use std::fmt::Debug;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

/// ASHv2 transport layer implementation.
#[derive(Debug)]
pub struct Ashv2<const BUF_SIZE: usize> {
    framed: Framed<AshFramed<BUF_SIZE>, RawCodec>,
    sequence: u8,
    buffer: Vec<u8>,
}

impl<const BUF_SIZE: usize> Ashv2<BUF_SIZE> {
    /// Creates an ASHv2 host.
    #[must_use]
    pub fn new(ash: AshFramed<BUF_SIZE>) -> Self {
        Self {
            framed: Framed::new(ash, RawCodec),
            sequence: 0,
            buffer: Vec::new(),
        }
    }
}

impl<const BUF_SIZE: usize> Transport for Ashv2<BUF_SIZE> {
    fn next_header<T>(&mut self, id: T::Size) -> Header<T>
    where
        T: ValidControl,
    {
        let header = Header::new(self.sequence, Control::<T>::default(), id);
        debug!("Header: {header:?}");
        self.sequence = self.sequence.wrapping_add(1);
        header
    }

    async fn send<C, P>(&mut self, command: P) -> Result<(), Error>
    where
        C: ValidControl,
        P: Parameter + ToLeStream,
        <P as Parameter>::Id: Into<C::Size>,
    {
        self.buffer.clear();
        let header = self.next_header::<C>(P::ID.into());
        self.buffer.extend(header.to_le_stream());
        self.buffer.extend(command.to_le_stream());
        debug!("Sending payload: {:#04X?}", self.buffer);
        self.framed.send(self.buffer.as_slice().into()).await?;
        Ok(())
    }

    async fn receive<C, P>(&mut self) -> Result<P, Error>
    where
        C: ValidControl,
        P: Clone + Debug + Send + Parameter + FromLeStream,
        <P as Parameter>::Id: Into<C::Size>,
    {
        if let Some(response) = self.framed.next().await {
            let response = response?;
            debug!("Received payload: {:#04X?}", response);
            let frame = parse_response::<Frame<C, P>>(response.as_ref())?;
            return Ok(frame.parameters());
        }

        Err(Error::Custom("foo".into()))
    }
}
