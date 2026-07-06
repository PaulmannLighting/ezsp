use std::pin::Pin;
use std::task::{Context, Poll};

use apis_saltans_hw::Error;
use tokio::sync::oneshot::Receiver;

use crate::ember::Status;

/// Response receiver for Ember responses.
#[derive(Debug)]
pub struct ResponseReceiver {
    inner: Receiver<Result<Status, u8>>,
    seq: u8,
}

impl ResponseReceiver {
    /// Create a new response receiver.
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
            Poll::Ready(Ok(other)) => Poll::Ready(Err(crate::Error::Status(other.into()).into())),
            Poll::Ready(Err(error)) => Poll::Ready(Err(error.into())),
        }
    }
}
