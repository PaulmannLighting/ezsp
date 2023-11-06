use crate::read_write::Readable;
use crate::Error;
use ashv2::{Event, HandleResult};
use log::error;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

#[derive(Debug)]
pub struct SingleFrameResponse<R>
where
    R: Debug + Readable,
{
    response: Arc<Mutex<Option<Result<R, Error>>>>,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl<R> SingleFrameResponse<R>
where
    R: Debug + Readable,
{
    pub fn new() -> Self {
        Self {
            response: Arc::new(Mutex::new(None)),
            waker: Arc::new(Mutex::new(None)),
        }
    }
}

impl<R> Clone for SingleFrameResponse<R>
where
    R: Debug + Readable,
{
    fn clone(&self) -> Self {
        Self {
            response: self.response.clone(),
            waker: self.waker.clone(),
        }
    }
}

impl<R> Default for SingleFrameResponse<R>
where
    R: Debug + Readable,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<R> Future for SingleFrameResponse<R>
where
    R: Debug + Readable,
{
    type Output = Result<R, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(response) = self
            .response
            .lock()
            .expect("Could not lock response.")
            .take()
        {
            Poll::Ready(response)
        } else {
            self.waker
                .lock()
                .expect("Could not lock waker.")
                .replace(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl<R> ashv2::Response<Arc<[u8]>> for SingleFrameResponse<R>
where
    R: Debug + Readable + Send + Sync + for<'a> TryFrom<&'a [u8]>,
    for<'a> <R as TryFrom<&'a [u8]>>::Error: std::error::Error,
{
    #[allow(clippy::unwrap_used)]
    fn handle(&self, event: Event<Result<Arc<[u8]>, ashv2::Error>>) -> HandleResult {
        let mut response = self.response.lock().unwrap();

        match event {
            Event::DataReceived(data) => match data {
                Ok(payload) => match R::try_from(payload.as_ref()) {
                    Ok(payload) => {
                        if response.is_some() {
                            HandleResult::Reject
                        } else {
                            response.replace(Ok(payload));
                            HandleResult::Completed
                        }
                    }
                    Err(error) => {
                        error!("{error}");
                        HandleResult::Reject
                    }
                },
                Err(error) => {
                    response.replace(Err(error.into()));
                    HandleResult::Completed
                }
            },
            Event::TransmissionCompleted => {
                if response.is_some() {
                    HandleResult::Completed
                } else {
                    HandleResult::Continue
                }
            }
        }
    }

    #[allow(clippy::unwrap_used)]
    fn abort(&self, error: ashv2::Error) {
        self.response.lock().unwrap().replace(Err(error.into()));
    }

    #[allow(clippy::unwrap_used)]
    fn wake(&self) {
        if let Some(waker) = self.waker.lock().unwrap().take() {
            waker.wake();
        }
    }
}
