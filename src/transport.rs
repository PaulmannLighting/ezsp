//! Transport layer to communicate with an NCP that supports the `EZSP` protocol.

use core::future::Future;
use std::num::NonZero;

use le_stream::ToLeStream;
use log::{info, trace, warn};

use crate::frame::{Parameter, RespondsWith};
use crate::parameters::configuration::version;
use crate::{Connection, Error, Parameters};

/// A transport layer to communicate with an NCP that supports the `EZSP` protocol.
///
/// This trait is intended to facilitate the implementation of the `EZSP` protocol,
/// since all `EZSP` traits have a blanket implementation for it.
///
/// Unless you know what you are doing, you should not use the methods of this trait directly.
pub trait Transport: Send {
    /// Initialize the `EZSP` connection.
    ///
    /// Negotiates the protocol version.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] on I/O errors or if the desired protocol version is not supported.
    fn connect(&mut self) -> impl Future<Output = Result<version::Response, Error>> + Send;

    /// Returns the current connection state.
    fn state(&self) -> Connection;

    /// Returns the negotiated protocol version, if any.
    fn negotiated_version(&self) -> Option<NonZero<u8>>;

    /// Send a command to the NCP.
    fn send<T>(&mut self, command: T) -> impl Future<Output = Result<u16, Error>> + Send
    where
        T: Parameter + ToLeStream;

    /// Receive a response from the NCP.
    fn receive<T>(&mut self) -> impl Future<Output = Result<T, Error>> + Send
    where
        T: TryFrom<Parameters> + Send,
        <T as TryFrom<Parameters>>::Error: Into<Parameters> + Send;

    /// Ensure that an `EZSP` connection is established and reset it if necessary.
    fn ensure_connection(&mut self) -> impl Future<Output = Result<(), Error>> + Send {
        async {
            match self.state() {
                Connection::Disconnected => {
                    info!("Initializing UART connection");
                    self.connect().await.map(drop)
                }
                Connection::Connected => {
                    trace!("UART is connected");
                    Ok(())
                }
                Connection::Failed => {
                    warn!("UART connection failed, reinitializing");
                    self.connect().await.map(drop)
                }
            }
        }
    }

    /// Communicate with the NCP.
    fn communicate<T>(
        &mut self,
        command: T,
    ) -> impl Future<Output = Result<T::Response, Error>> + Send
    where
        T: Parameter + RespondsWith + ToLeStream,
    {
        async {
            self.ensure_connection().await?;
            self.send(command).await?;
            self.receive().await
        }
    }
}
