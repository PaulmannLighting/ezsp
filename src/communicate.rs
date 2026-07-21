//! High-level command/response communication over an EZSP transport.

use core::future::Future;

use le_stream::ToLeStream;

use crate::Error;
use crate::frame::{Commands, Parameter, RespondsWith};

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
        T: Parameter + RespondsWith + ToLeStream + Into<Commands>;
}
