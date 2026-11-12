//! Scripted command transactions for protocol-level regression tests.

use std::collections::VecDeque;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

use le_stream::ToLeStream;

use crate::frame::{Commands, Parameter, RespondsWith};
use crate::{Communicate, Error, Parameters, Parsable};

/// One expected command and its raw response or transport failure.
pub struct Transaction {
    id: u16,
    request: Vec<u8>,
    response: Result<Vec<u8>, Error>,
}

impl Transaction {
    /// Expect a command and return the given response payload.
    pub fn reply(id: u16, request: &[u8], response: &[u8]) -> Self {
        Self {
            id,
            request: request.to_vec(),
            response: Ok(response.to_vec()),
        }
    }

    /// Expect a command and report a closed response channel.
    pub fn fail(id: u16, request: &[u8]) -> Self {
        Self {
            id,
            request: request.to_vec(),
            response: Err(Error::ChannelClosed),
        }
    }
}

/// A communicator that checks every command against an ordered script.
pub struct Script(VecDeque<Transaction>);

impl Script {
    /// Create an ordered transaction script.
    pub fn new<T>(transactions: T) -> Self
    where
        T: IntoIterator<Item = Transaction>,
    {
        Self(transactions.into_iter().collect())
    }

    /// Assert that all expected commands were consumed.
    pub fn assert_finished(&self) {
        assert!(self.0.is_empty(), "not all expected commands were issued");
    }
}

impl Communicate for Script {
    #[expect(
        clippy::unused_async_trait_impl,
        reason = "scripted transactions complete immediately"
    )]
    async fn communicate<T>(&mut self, command: T) -> Result<T::Response, Error>
    where
        T: Parameter + RespondsWith + ToLeStream + Into<Commands>,
    {
        let transaction = self.0.pop_front().expect("unexpected command");
        assert_eq!(T::ID, transaction.id);
        assert_eq!(
            command.to_le_stream().collect::<Vec<_>>(),
            transaction.request
        );
        let parameters =
            Parameters::parse_from_le_stream(T::ID, transaction.response?.into_iter())?;
        T::Response::try_from(parameters)
            .map_err(|error| Error::UnexpectedResponse(Box::new(error.into())))
    }
}

/// Poll a scripted future once, rejecting unexpected suspension.
pub fn ready<F>(future: F) -> F::Output
where
    F: Future,
{
    let mut future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());
    match future.as_mut().poll(&mut context) {
        Poll::Ready(output) => output,
        Poll::Pending => panic!("scripted future unexpectedly blocked"),
    }
}
