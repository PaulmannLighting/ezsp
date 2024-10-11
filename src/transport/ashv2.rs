use crate::frame::{Control, Header, Parameter};
use crate::transport::parse_response::parse_response;
use crate::transport::Transport;
use crate::Error;
use ashv2::AsyncAsh;
use le_stream::{FromLeStream, ToLeStream};
use log::debug;
use std::fmt::Debug;

/// ASHv2 transport layer implementation.
#[derive(Debug)]
pub struct Ashv2<T>
where
    T: AsyncAsh,
{
    host: T,
    sequence: u8,
    control: Control,
    buffer: Vec<u8>,
}

impl<T> Ashv2<T>
where
    T: AsyncAsh,
{
    /// Creates an ASHv2 host.
    #[must_use]
    pub const fn new(host: T, control: Control) -> Self {
        Self {
            host,
            sequence: 0,
            control,
            buffer: Vec::new(),
        }
    }
}

impl<T> Transport for Ashv2<T>
where
    T: AsyncAsh + Send,
{
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
        let response = self.host.communicate(&self.buffer).await?;
        parse_response::<R>(&response)
    }
}
