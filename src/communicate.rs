//! High-level command/response communication over an EZSP transport.

use core::future::Future;
use std::sync::Arc;

use le_stream::ToLeStream;
use log::{info, trace, warn};
use tokio::sync::Mutex;

use crate::frame::{Parameter, RespondsWith};
use crate::{Connection, Error, Transport};

/// Sends typed EZSP commands and receives their responses.
///
/// The command-group traits are blanket-implemented for communicators. Every
/// [`Transport`] receives this implementation automatically, while alternate
/// implementations may provide the same transaction interface without
/// exposing the lower-level transport operations.
///
/// `Arc<tokio::sync::Mutex<T>>` also implements this trait when `T` does. That
/// implementation holds the mutex across the complete command/response
/// transaction so multiple owners cannot interleave responses.
pub trait Communicate: Send {
    /// Ensures that the EZSP connection is established, reconnecting if needed.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if connection initialization fails.
    fn ensure_connection(&mut self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sends one command and waits for its typed response.
    ///
    /// This method does not establish the connection automatically. Call
    /// [`Communicate::ensure_connection`] when connection setup is required.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if sending the command or receiving its response
    /// fails.
    fn communicate<T>(
        &mut self,
        command: T,
    ) -> impl Future<Output = Result<T::Response, Error>> + Send
    where
        T: Parameter + RespondsWith + ToLeStream;
}

impl<T> Communicate for T
where
    T: Transport,
{
    async fn ensure_connection(&mut self) -> Result<(), Error> {
        match self.state() {
            Connection::Disconnected => {
                info!("Initializing transport connection");
                self.connect().await.map(drop)
            }
            Connection::Connected => {
                trace!("Transport is connected");
                Ok(())
            }
            Connection::Failed => {
                warn!("Transport connection failed, reinitializing");
                self.connect().await.map(drop)
            }
        }
    }

    async fn communicate<U>(&mut self, command: U) -> Result<U::Response, Error>
    where
        U: Parameter + RespondsWith + ToLeStream,
    {
        self.send(command).await?;
        self.receive().await
    }
}

impl<T> Communicate for Arc<Mutex<T>>
where
    T: Communicate,
{
    async fn ensure_connection(&mut self) -> Result<(), Error> {
        self.lock().await.ensure_connection().await
    }

    async fn communicate<U>(&mut self, command: U) -> Result<U::Response, Error>
    where
        U: Parameter + RespondsWith + ToLeStream,
    {
        self.lock().await.communicate(command).await
    }
}
