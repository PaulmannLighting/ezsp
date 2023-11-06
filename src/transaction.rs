use crate::read_write::{Readable, Writable};
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

#[derive(Debug)]
pub struct Transaction<C, R>
where
    C: Debug + Writable,
    R: Debug + Readable,
{
    command: C,
    response: Response<R>,
}

impl<C, R> Transaction<C, R>
where
    C: Debug + Writable,
    R: Debug + Readable,
{
    #[must_use]
    pub fn new(command: C) -> Self {
        Self {
            command,
            response: Response::<R>::new(),
        }
    }

    #[must_use]
    pub fn clone_response(&self) -> Response<R> {
        //self.response.clone()
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct Response<R>
where
    R: Debug + Readable,
{
    response: Arc<Mutex<Option<R>>>,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl<R> Response<R>
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

impl<R> Default for Response<R>
where
    R: Debug + Readable,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<R> Future for Response<R>
where
    R: Debug + Readable,
{
    type Output = R;

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
