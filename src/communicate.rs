//! High-level command/response communication over an EZSP transport.

use core::future::Future;

use le_stream::ToLeStream;

use crate::Error;
use crate::frame::{Commands, Parameter, RespondsWith};

/// Sends typed EZSP commands and receives their responses.
///
/// The command-group traits are blanket-implemented for every communicator.
/// [`Connection`](crate::Connection) is the standard implementation: it sends a
/// command message to the transmitter actor and awaits the response correlated
/// with that command's EZSP sequence number. Alternative transports can expose
/// the same typed transaction interface by implementing this trait.
pub trait Communicate: Send {
    /// Sends one command and waits for its typed response.
    ///
    /// Connection negotiation is a separate lifecycle step. The standard actor
    /// API only yields a [`Connection`](crate::Connection) communicator after
    /// [`Client::connect`](crate::Client::connect) succeeds.
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
