use crate::frame::{Codec, Control, Frame, Header, Parameter, ValidControl};
use crate::transport::Transport;
use crate::Error;
use ashv2::AshFramed;
use futures::{SinkExt, StreamExt};
use le_stream::{FromLeStream, ToLeStream};
use std::fmt::Debug;
use tokio_util::codec::Framed;

/// An `EZSP` host using `ASHv2` on the transport layer.
#[derive(Debug)]
pub struct Ashv2<const BUF_SIZE: usize> {
    ash: AshFramed<BUF_SIZE>,
    sequence: u8,
}

impl<const BUF_SIZE: usize> Ashv2<BUF_SIZE> {
    /// Creates an ASHv2 host.
    #[must_use]
    pub const fn new(ash: AshFramed<BUF_SIZE>) -> Self {
        Self { ash, sequence: 0 }
    }

    fn framed<C, P>(&self) -> Framed<&AshFramed<BUF_SIZE>, Codec<C, P>>
    where
        C: ValidControl,
        P: Parameter,
        <P as Parameter>::Id: Into<C::Size>,
    {
        Framed::new(&self.ash, Frame::<C, P>::codec())
    }
}

impl<const BUF_SIZE: usize> Transport for Ashv2<BUF_SIZE> {
    fn next_header<T>(&mut self, id: T::Size) -> Header<T>
    where
        T: ValidControl,
    {
        let header = Header::new(self.sequence, Control::<T>::default(), id);
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
        self.framed().send(Frame::new(header, command)).await?;
        Ok(())
    }

    async fn receive<C, P>(&mut self) -> Result<P, Error>
    where
        C: ValidControl,
        P: Clone + Debug + Parameter + FromLeStream,
        <P as Parameter>::Id: Into<C::Size>,
    {
        let Some(response) = self.framed::<C, P>().next().await else {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "No more data to construct frame.",
            )));
        };

        Ok(response?.parameters())
    }
}
