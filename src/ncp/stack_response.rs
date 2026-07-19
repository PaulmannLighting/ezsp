use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::sync::oneshot::Receiver;

use crate::Error;
use crate::ember::Status;

/// A deferred stack response for an outgoing APS message.
///
/// The NCP send helpers return this value after the EZSP send command has been
/// accepted. Awaiting it returns `Ok(())` when the corresponding `messageSent`
/// callback reports success. A failure status, including an unknown raw status
/// value, is converted into an [`Error`]. Dropping a `StackResponse` discards
/// only the callback notification; it does not cancel an APS message that has
/// already been accepted by the NCP.
///
/// # Panics
///
/// Awaiting the `StackResponse` panics if the callback handler drops its
/// response sender without reporting a stack status.
#[must_use = "futures do nothing unless polled"]
#[derive(Debug)]
pub struct StackResponse {
    inner: Receiver<Result<Status, u8>>,
}

impl From<Receiver<Result<Status, u8>>> for StackResponse {
    fn from(stack_response: Receiver<Result<Status, u8>>) -> Self {
        Self {
            inner: stack_response,
        }
    }
}

impl Future for StackResponse {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        match Pin::new(&mut this.inner).poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(Ok(Status::Success))) => Poll::Ready(Ok(())),
            Poll::Ready(Ok(response)) => Poll::Ready(Err(response.into())),
            Poll::Ready(Err(error)) => {
                panic!("stack response channel closed before a status was reported: {error}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::task::Waker;

    use tokio::sync::oneshot::channel;

    use super::*;
    use crate::error::Status as ErrorStatus;

    const UNKNOWN_STATUS: u8 = 0xFE;

    #[test]
    fn remains_pending_until_callback_response_arrives() {
        let (_sender, receiver) = channel();
        let mut stack_response = Box::pin(StackResponse::from(receiver));
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);

        assert!(matches!(
            stack_response.as_mut().poll(&mut context),
            Poll::Pending
        ));
    }

    #[test]
    fn returns_unit_for_successful_stack_response() {
        let (sender, receiver) = channel();
        sender.send(Ok(Status::Success)).expect("receiver is open");
        let mut stack_response = Box::pin(StackResponse::from(receiver));
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);

        assert!(matches!(
            stack_response.as_mut().poll(&mut context),
            Poll::Ready(Ok(()))
        ));
    }

    #[test]
    fn converts_known_stack_failure_to_error() {
        let (sender, receiver) = channel();
        sender
            .send(Ok(Status::DeliveryFailed))
            .expect("receiver is open");
        let mut stack_response = Box::pin(StackResponse::from(receiver));
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);

        assert!(matches!(
            stack_response.as_mut().poll(&mut context),
            Poll::Ready(Err(Error::Status(ErrorStatus::Ember(Ok(
                Status::DeliveryFailed
            )))))
        ));
    }

    #[test]
    fn converts_unknown_stack_status_to_error() {
        let (sender, receiver) = channel();
        sender.send(Err(UNKNOWN_STATUS)).expect("receiver is open");
        let mut stack_response = Box::pin(StackResponse::from(receiver));
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);

        assert!(matches!(
            stack_response.as_mut().poll(&mut context),
            Poll::Ready(Err(Error::Status(ErrorStatus::Ember(Err(UNKNOWN_STATUS)))))
        ));
    }

    #[test]
    #[should_panic(expected = "stack response channel closed before a status was reported")]
    fn panics_when_callback_response_sender_is_dropped() {
        let (sender, receiver) = channel();
        drop(sender);
        let mut stack_response = Box::pin(StackResponse::from(receiver));
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);

        _ = stack_response.as_mut().poll(&mut context);
    }
}
