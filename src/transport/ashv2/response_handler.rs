use crate::frame::Frame;
use crate::Error;
use ashv2::{Event, HandleResult, Handler, Response};
use le_stream::{FromLeBytes, ToLeBytes};
use log::{debug, warn};
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, MutexGuard};
use std::task::{Context, Poll, Waker};

type ResultType<C, I, P> = Result<Frame<C, I, P>, Error>;

#[derive(Clone, Debug)]
pub struct ResponseHandler<C, I, P>
where
    C: Copy + Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
    I: Copy + Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
    P: FromLeBytes + ToLeBytes,
{
    waker: Arc<Mutex<Option<Waker>>>,
    buffer: Arc<Mutex<Vec<u8>>>,
    result: Arc<Mutex<Option<ResultType<C, I, P>>>>,
}

impl<C, I, P> ResponseHandler<C, I, P>
where
    C: Copy + Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
    I: Copy + Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
    P: FromLeBytes + ToLeBytes,
{
    #[must_use]
    pub const fn new(
        waker: Arc<Mutex<Option<Waker>>>,
        buffer: Arc<Mutex<Vec<u8>>>,
        result: Arc<Mutex<Option<ResultType<C, I, P>>>>,
    ) -> Self {
        Self {
            waker,
            buffer,
            result,
        }
    }

    #[allow(clippy::significant_drop_tightening)]
    fn try_parse(&self) -> HandleResult {
        let buffer = self.buffer();
        let mut bytes = buffer.iter().copied();

        Frame::from_le_bytes(&mut bytes).map_or_else(
            |_| {
                self.replace_result(Err("Incomplete data".to_string().into()));
                HandleResult::Continue
            },
            |frame| {
                self.replace_result(Ok(frame));

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

    fn extend_buffer(&self, bytes: &[u8]) {
        debug!("Locking buffer.");
        self.buffer().extend_from_slice(bytes);
        debug!("Releasing lock on buffer.");
    }

    fn result(&self) -> MutexGuard<'_, Option<ResultType<C, I, P>>> {
        self.result
            .lock()
            .expect("Result should never be poisoned.")
    }

    fn replace_result(&self, result: ResultType<C, I, P>) {
        debug!("Locking result.");
        self.result().replace(result);
        debug!("Releasing lock on result.");
    }

    fn result_is_none(&self) -> bool {
        debug!("Locking result.");
        let is_some = self.result().is_none();
        debug!("Releasing lock on result.");
        is_some
    }
}

impl<C, I, P> Default for ResponseHandler<C, I, P>
where
    C: Copy + Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
    I: Copy + Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
    P: FromLeBytes + ToLeBytes,
{
    fn default() -> Self {
        Self::new(
            Arc::new(Mutex::new(None)),
            Arc::new(Mutex::new(Vec::new())),
            Arc::new(Mutex::new(None)),
        )
    }
}

impl<C, I, P> Future for ResponseHandler<C, I, P>
where
    C: Copy + Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
    I: Copy + Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
    P: FromLeBytes + ToLeBytes,
{
    type Output = Result<P, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Ok(mut result) = self.result.lock() {
            if let Some(result) = result.take() {
                return Poll::Ready(result.map(Frame::parameters));
            }
        }

        if let Ok(mut waker) = self.waker.lock() {
            waker.replace(cx.waker().clone());
        }

        Poll::Pending
    }
}

impl<C, I, P> Handler<Arc<[u8]>> for ResponseHandler<C, I, P>
where
    C: Copy + Debug + Eq + PartialEq + Send + Sync + FromLeBytes + ToLeBytes,
    I: Copy + Debug + Eq + PartialEq + Send + Sync + FromLeBytes + ToLeBytes,
    P: Debug + Send + Sync + FromLeBytes + ToLeBytes,
{
    fn handle(&self, event: Event<Result<Arc<[u8]>, ashv2::Error>>) -> HandleResult {
        match event {
            Event::DataReceived(data) => match data {
                Ok(bytes) => {
                    if self.result_is_none() {
                        self.extend_buffer(&bytes);
                        self.try_parse()
                    } else {
                        HandleResult::Completed
                    }
                }
                Err(error) => {
                    if self.result_is_none() {
                        self.replace_result(Err(error.into()));
                        HandleResult::Failed
                    } else {
                        HandleResult::Completed
                    }
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
        self.replace_result(Err(error.into()));
    }

    fn wake(&self) {
        if let Ok(mut waker) = self.waker.lock() {
            if let Some(waker) = waker.take() {
                waker.wake();
            }
        }
    }
}

impl<C, I, P> Response for ResponseHandler<C, I, P>
where
    C: Copy + Debug + Eq + PartialEq + Send + Sync + FromLeBytes + ToLeBytes,
    I: Copy + Debug + Eq + PartialEq + Send + Sync + FromLeBytes + ToLeBytes,
    P: Clone + Debug + Send + Sync + FromLeBytes + ToLeBytes,
{
    type Result = P;
    type Error = Error;
}
