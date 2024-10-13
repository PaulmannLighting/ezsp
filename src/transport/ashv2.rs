use crate::frame::{Control, Frame, Header, Parameter, ValidControl};
use crate::transport::Transport;
use crate::Error;
use ashv2::AshFramed;
use codec::Codec;
use futures::{SinkExt, StreamExt};
use le_stream::{FromLeStream, ToLeStream};
use std::fmt::Debug;
use std::io::ErrorKind;
use tokio_util::codec::Framed;

pub mod codec;

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

    fn framed<C, P>(&mut self) -> Framed<&mut AshFramed<BUF_SIZE>, Codec<C, P>>
    where
        C: ValidControl,
        P: Parameter,
        <C as ValidControl>::Size: From<<P as Parameter>::Id>,
    {
        Framed::new(&mut self.ash, Codec::default())
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
        <C as ValidControl>::Size: From<<P as Parameter>::Id>,
    {
        let header = self.next_header::<C>(<C as ValidControl>::Size::from(P::ID));
        self.framed().send(Frame::new(header, command)).await?;
        Ok(())
    }

    async fn receive<C, P>(&mut self) -> Result<P, Error>
    where
        C: ValidControl,
        P: Parameter + FromLeStream,
        <C as ValidControl>::Size: From<<P as Parameter>::Id>,
    {
        let Some(response) = self.framed::<C, P>().next().await else {
            return Err(
                std::io::Error::new(ErrorKind::UnexpectedEof, "Empty response from NCP.").into(),
            );
        };

        Ok(response?.parameters())
    }
}
