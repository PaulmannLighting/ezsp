use crate::frame::ResponseFrame;
use crate::transport::Error;
use ashv2::{Event, HandleResult, Handler, Response};
use le_stream::FromLeBytes;
use log::warn;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, MutexGuard};
use std::task::{Context, Poll, Waker};

type ResultType<P> = Option<Result<ResponseFrame<P>, Error>>;

#[derive(Clone, Debug)]
pub struct ResponseHandler<P>
where
    P: FromLeBytes,
{
    waker: Arc<Mutex<Option<Waker>>>,
    buffer: Arc<Mutex<Vec<u8>>>,
    result: Arc<Mutex<ResultType<P>>>,
}

impl<P> ResponseHandler<P>
where
    P: FromLeBytes,
{
    #[must_use]
    pub const fn new(
        waker: Arc<Mutex<Option<Waker>>>,
        buffer: Arc<Mutex<Vec<u8>>>,
        result: Arc<Mutex<ResultType<P>>>,
    ) -> Self {
        Self {
            waker,
            buffer,
            result,
        }
    }

    fn try_parse(&self) -> HandleResult {
        let buffer = self.buffer();
        let mut bytes = buffer.iter().copied();

        ResponseFrame::from_le_bytes(&mut bytes).map_or_else(
            |_| {
                self.result()
                    .replace(Err("Incomplete data".to_string().into()));
                HandleResult::Continue
            },
            |frame| {
                self.result().replace(Ok(frame));

                for byte in bytes {
                    warn!("Found excess byte in response: {byte:?}");
                }

                HandleResult::Completed
            },
        )
    }

    fn buffer(&self) -> MutexGuard<'_, Vec<u8>> {
        self.buffer
            .lock()
            .expect("Buffer should never be poisoned.")
    }

    fn result(&self) -> MutexGuard<'_, Option<Result<ResponseFrame<P>, Error>>> {
        self.result
            .lock()
            .expect("Result should never be poisoned.")
    }
}

impl<P> Default for ResponseHandler<P>
where
    P: FromLeBytes,
{
    fn default() -> Self {
        Self::new(
            Arc::new(Mutex::new(None)),
            Arc::new(Mutex::new(Vec::new())),
            Arc::new(Mutex::new(None)),
        )
    }
}

impl<P> Future for ResponseHandler<P>
where
    P: FromLeBytes,
{
    type Output = Result<P, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Ok(mut result) = self.result.lock() {
            if let Some(result) = result.take() {
                return Poll::Ready(result.map(ResponseFrame::parameters));
            }
        }

        if let Ok(mut waker) = self.waker.lock() {
            waker.replace(cx.waker().clone());
        }

        Poll::Pending
    }
}

impl<P> Handler<Arc<[u8]>> for ResponseHandler<P>
where
    P: FromLeBytes + Debug + Send + Sync,
{
    fn handle(&self, event: Event<Result<Arc<[u8]>, ashv2::Error>>) -> HandleResult {
        match event {
            Event::DataReceived(data) => match data {
                Ok(bytes) => {
                    self.buffer().extend_from_slice(&bytes);
                    self.try_parse()
                }
                Err(error) => {
                    self.result().replace(Err(error.into()));
                    HandleResult::Reject
                }
            },
            Event::TransmissionCompleted => {
                self.result()
                    .as_ref()
                    .map_or(HandleResult::Failed, |result| {
                        if result.is_ok() {
                            HandleResult::Completed
                        } else {
                            HandleResult::Failed
                        }
                    })
            }
        }
    }

    fn abort(&self, error: ashv2::Error) {
        self.result().replace(Err(error.into()));
    }

    fn wake(&self) {
        if let Ok(mut waker) = self.waker.lock() {
            if let Some(waker) = waker.take() {
                waker.wake();
            }
        }
    }
}

impl<P> Response for ResponseHandler<P>
where
    P: FromLeBytes + Debug + Send + Sync,
{
    type Result = P;
    type Error = Error;
}
