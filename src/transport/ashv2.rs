mod codec;

use crate::frame::{Control, Frame, Header, Parameter, ValidControl};
use crate::transport::Transport;
use crate::Error;
use ashv2::AshFramed;
use codec::RawCodec;
use futures::{SinkExt, StreamExt};
use le_stream::{FromLeStream, ToLeStream};
use log::debug;
use std::fmt::Debug;
use tokio_util::codec::Framed;

/// ASHv2 transport layer implementation.
#[derive(Debug)]
pub struct Ashv2<const BUF_SIZE: usize> {
    ash: AshFramed<BUF_SIZE>,
    sequence: u8,
    buffer: Vec<u8>,
}

impl<const BUF_SIZE: usize> Ashv2<BUF_SIZE> {
    /// Creates an ASHv2 host.
    #[must_use]
    pub const fn new(ash: AshFramed<BUF_SIZE>) -> Self {
        Self {
            ash,
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
        debug!(
            "Header: {:#04X?}",
            header.to_le_stream().collect::<Vec<u8>>()
        );
        self.sequence = self.sequence.wrapping_add(1);
        header
    }

    async fn send<C, P>(&mut self, command: P) -> Result<(), Error>
    where
        C: ValidControl,
        P: Parameter + ToLeStream,
        <P as Parameter>::Id: Into<C::Size>,
    {
        let header = self.next_header::<C>(P::ID.into());
        debug!("Sending payload: {:#04X?}", self.buffer);
        Framed::new(&self.ash, Frame::<C, P>::codec())
            .send(Frame::new(header, command))
            .await?;
        Ok(())
    }

    async fn receive_raw<C, R>(&mut self) -> Result<R, Error>
    where
        C: ValidControl,
        R: Clone + Debug + FromLeStream,
    {
        if let Some(response) = Framed::new(&self.ash, RawCodec).next().await {
            let response = response?;
            debug!("Received payload: {:#04X?}", response);
            let frame = R::from_le_stream_exact(response.iter().copied())?;
            return Ok(frame);
        }

        Err(Error::Custom("no more data".into()))
    }

    async fn receive<C, P>(&mut self) -> Result<P, Error>
    where
        C: ValidControl,
        P: Clone + Debug + Parameter + FromLeStream,
        <P as Parameter>::Id: Into<C::Size>,
    {
        if let Some(response) = Framed::new(&self.ash, Frame::<C, P>::codec()).next().await {
            let frame = response?;
            debug!("Received payload: {frame:#04X?}");
            return Ok(frame.parameters());
        }

        Err(Error::Io(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "No more data to construct frame.",
        )))
    }
}
