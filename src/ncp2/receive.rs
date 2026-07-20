use core::future::Future;

use crate::{Error, Parameters, Transport};

/// Receives raw EZSP response or callback parameters.
///
/// `None` indicates that the incoming frame stream has closed. Received values
/// can be forwarded to an [`Ncp`](super::Ncp) through
/// [`Message::Received`](super::Message::Received).
pub trait Receive: Send {
    /// Receives the next raw EZSP frame, or `None` when the stream closes.
    fn receive(&mut self) -> impl Future<Output = Option<Result<Parameters, Error>>> + Send;
}

impl<T> Receive for T
where
    T: Transport,
{
    async fn receive(&mut self) -> Option<Result<Parameters, Error>> {
        match Transport::receive::<Parameters>(self).await {
            Err(Error::ChannelClosed) => None,
            result => Some(result),
        }
    }
}
