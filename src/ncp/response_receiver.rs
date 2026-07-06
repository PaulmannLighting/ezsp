use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::sync::oneshot::Receiver;

use crate::Error;
use crate::ember::Status;

/// Future resolving an outgoing APS message confirmation.
///
/// `Ncp::unicast` and `Ncp::multicast` return this value after the immediate
/// send command succeeds. Awaiting it waits for the matching `messageSent`
/// callback and resolves to the APS sequence number on success.
#[derive(Debug)]
pub struct ResponseReceiver {
    inner: Receiver<Result<Status, u8>>,
    seq: u8,
}

impl ResponseReceiver {
    /// Creates a new response receiver from a `messageSent` status channel and APS sequence.
    #[must_use]
    pub const fn new(inner: Receiver<Result<Status, u8>>, seq: u8) -> Self {
        Self { inner, seq }
    }
}

impl Future for ResponseReceiver {
    type Output = Result<u8, Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.inner).poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(Ok(Status::Success))) => Poll::Ready(Ok(self.seq)),
            Poll::Ready(Ok(other)) => Poll::Ready(Err(Error::Status(other.into()))),
            Poll::Ready(Err(error)) => Poll::Ready(Err(error.into())),
        }
    }
}
