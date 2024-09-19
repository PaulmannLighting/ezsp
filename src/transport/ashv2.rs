use std::fmt::Debug;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::mpsc::Sender;

use ashv2::{FrameBuffer, Host};
use le_stream::{FromLeStream, ToLeStream};
use log::debug;
use serialport::TTYPort;

use crate::frame::{Control, Header, Parameter};
use crate::transport::ashv2::response_handler::ResponseHandler;
use crate::transport::Transport;
use crate::Error;

mod response_handler;

/// ASHv2 transport layer implementation.
#[derive(Debug)]
pub struct Ashv2 {
    host: Host,
    sequence: AtomicU8,
    control: Control,
}

impl Ashv2 {
    /// Spawns an ASHv2 host.
    ///
    /// # Errors
    /// Returns an [`ashv2::Error`] if spawning fails.
    pub fn spawn(
        serial_port: TTYPort,
        control: Control,
        callback: Option<Sender<FrameBuffer>>,
    ) -> Result<Self, ashv2::Error> {
        Ok(Self {
            host: Host::spawn(serial_port, callback)?,
            sequence: AtomicU8::new(0),
            control,
        })
    }
}

impl Transport for Ashv2 {
    fn next_header<R>(&self) -> Header<R::Id>
    where
        R: Parameter,
    {
        let sequence = self.sequence.load(SeqCst);
        let header = Header::new(sequence, self.control.into(), R::ID);
        debug!("Header: {header:?}");
        self.sequence
            .store(sequence.checked_add(1).unwrap_or(0), SeqCst);
        header
    }

    async fn communicate<C, R>(&self, command: C) -> Result<R, Error>
    where
        C: Parameter + ToLeStream,
        R: Clone + Debug + Send + Sync + Parameter + FromLeStream + 'static,
    {
        let mut payload = Vec::new();
        payload.extend(self.next_header::<R>().to_le_stream());
        payload.extend(command.to_le_stream());
        debug!("Sending payload: {payload:?}");
        self.host.communicate::<ResponseHandler<R>>(&payload).await
    }
}
