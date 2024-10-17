use std::fmt::Debug;
use std::hash::Hash;
use std::io::ErrorKind;

use futures::{SinkExt, StreamExt};
use le_stream::{FromLeStream, ToLeStream};
use tokio_util::codec::Framed;

use crate::error::{Decode, Error};
use crate::frame::{Command, Frame, Header, Parameter};
use crate::transport::Transport;

use ashv2::AshFramed;
pub use callbacks::Callbacks;
use codec::Codec;

mod callbacks;
mod codec;

/// An `EZSP` host using `ASHv2` on the transport layer.
#[derive(Debug)]
pub struct Ashv2<const BUF_SIZE: usize> {
    ash: AshFramed<BUF_SIZE>,
    sequence: u8,
}

impl<const BUF_SIZE: usize> Ashv2<BUF_SIZE> {
    /// Creates an `ASHv2` host.
    #[must_use]
    pub const fn new(ash: AshFramed<BUF_SIZE>) -> Self {
        Self { ash, sequence: 0 }
    }

    fn framed<H, P>(&mut self) -> Framed<&mut AshFramed<BUF_SIZE>, Codec<H, P>>
    where
        H: Header<P::Id>,
        P: Parameter,
    {
        Framed::new(&mut self.ash, Codec::default())
    }
}

impl<const BUF_SIZE: usize> Transport for Ashv2<BUF_SIZE> {
    fn next_header<H, T>(&mut self, id: T) -> H
    where
        H: Header<T>,
        T: Copy + Clone + Debug + Eq + Hash + Into<u16> + PartialEq + Send,
    {
        let header = H::new(self.sequence, Command::default().into(), id);
        self.sequence = self.sequence.wrapping_add(1);
        header
    }

    async fn send<H, P>(&mut self, command: P) -> Result<(), Error>
    where
        H: Header<P::Id>,
        P: Parameter + ToLeStream,
    {
        let Some(id) = P::ID else {
            return Err(Decode::MissingId.into());
        };

        let header = self.next_header::<H, P::Id>(id);
        self.framed().send(Frame::new(header, command)).await
    }

    async fn receive<H, P>(&mut self) -> Result<P, Error>
    where
        H: Header<P::Id> + Send,
        P: Parameter + FromLeStream,
    {
        let Some(response) = self.framed::<H, P>().next().await else {
            return Err(
                std::io::Error::new(ErrorKind::UnexpectedEof, "Empty response from NCP.").into(),
            );
        };

        Ok(response?.parameters())
    }
}
