use crate::frame::parameters::utilities::invalid_command;
use crate::frame::{Frame, Parameter};
use crate::{Error, Header};
use ashv2::{Event, HandleResult, Handler, Response};
use le_stream::FromLeBytes;
use log::{debug, error, warn};
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, MutexGuard};
use std::task::{Context, Poll, Waker};

type ResultType<R> = Result<Frame<R>, Error>;

#[derive(Clone, Debug)]
pub struct ResponseHandler<R>
where
    R: Parameter,
{
    waker: Arc<Mutex<Option<Waker>>>,
    buffer: Arc<Mutex<Vec<u8>>>,
    result: Arc<Mutex<Option<ResultType<R>>>>,
}

impl<R> ResponseHandler<R>
where
    R: Parameter + FromLeBytes,
{
    #[must_use]
    pub const fn new(
        waker: Arc<Mutex<Option<Waker>>>,
        buffer: Arc<Mutex<Vec<u8>>>,
        result: Arc<Mutex<Option<ResultType<R>>>>,
    ) -> Self {
        Self {
            waker,
            buffer,
            result,
        }
    }

    fn try_parse(&self) -> HandleResult {
        let buffer = self.buffer();
        let mut bytes = buffer.iter().copied().enumerate().map(|(index, byte)| {
            debug!("Byte #{index}: {byte:?}");
            byte
        });

        match Self::parse_header(&mut bytes) {
            Ok(header) => match R::from_le_bytes(&mut bytes) {
                Ok(parameters) => {
                    self.replace_result(Ok(Frame::new(header, parameters)));

                    for byte in bytes {
                        warn!("Found excess byte in response: {byte:?}");
                    }

                    HandleResult::Completed
                }
                Err(error) => {
                    error!("Error: {error}");
                    self.replace_result(Err("Incomplete data".to_string().into()));
                    HandleResult::Continue
                }
            },
            Err(error) => {
                drop(buffer);
                self.result().replace(Err(error));
                HandleResult::Failed
            }
        }
    }

    fn parse_header<T>(bytes: &mut T) -> Result<Header<R::Id>, Error>
    where
        T: Iterator<Item = u8>,
    {
        let header = Header::from_le_bytes(bytes)?;

        if header.id() == R::ID {
            Ok(header)
        } else if Into::<u16>::into(header.id()) == invalid_command::Response::ID {
            Err(Error::InvalidCommand(
                invalid_command::Response::from_le_bytes(bytes)?,
            ))
        } else {
            error!(
                "Invalid frame id. Expected {}, but got {}.",
                R::ID,
                header.id()
            );
            Err(Error::InvalidHeader {
                expected: Into::<u16>::into(R::ID),
                found: Into::<u16>::into(header.id()),
            })
        }
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

    fn result(&self) -> MutexGuard<'_, Option<ResultType<R>>> {
        self.result
            .lock()
            .expect("Result should never be poisoned.")
    }

    fn replace_result(&self, result: ResultType<R>) {
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

impl<R> Default for ResponseHandler<R>
where
    R: Parameter + FromLeBytes,
{
    fn default() -> Self {
        Self::new(
            Arc::new(Mutex::new(None)),
            Arc::new(Mutex::new(Vec::new())),
            Arc::new(Mutex::new(None)),
        )
    }
}

impl<R> Future for ResponseHandler<R>
where
    R: Parameter,
{
    type Output = Result<R, Error>;

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

impl<R> Handler for ResponseHandler<R>
where
    R: Debug + Send + Sync + Parameter + FromLeBytes,
{
    fn handle(&self, event: Event) -> HandleResult {
        match event {
            Event::DataReceived(bytes) => {
                if self.result_is_none() {
                    self.extend_buffer(bytes);
                    self.try_parse()
                } else {
                    HandleResult::Completed
                }
            }
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

impl<R> Response for ResponseHandler<R>
where
    R: Clone + Debug + Send + Sync + Parameter + FromLeBytes,
{
    type Result = R;
    type Error = Error;
}
