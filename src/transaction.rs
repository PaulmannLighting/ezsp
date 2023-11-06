mod single_frame_response;

use crate::read_write::{Readable, Writable};
use single_frame_response::SingleFrameResponse;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Transaction<C, R>
where
    C: Debug + Writable,
    R: Debug + Readable + Send + Sync,
{
    command: C,
    response: SingleFrameResponse<R>,
}

impl<C, R> Transaction<C, R>
where
    C: Debug + Writable,
    R: Debug + Readable + Send + Sync,
{
    #[must_use]
    pub fn new(command: C) -> Self {
        Self {
            command,
            response: SingleFrameResponse::<R>::new(),
        }
    }

    #[must_use]
    pub fn clone_response(&self) -> SingleFrameResponse<R> {
        self.response.clone()
    }
}
