use std::io;
use std::io::ErrorKind;

use tokio_mpmc::Receiver;

use crate::ember::Status;
use crate::frame::parameters::networking::handler::Handler as NetworkingEvent;
use crate::{Callback, Error};

/// Trait for checking the stack status.
pub trait StackStatus {
    /// Return the current stack status as a string.
    fn stack_status(&mut self, target_status: Status) -> impl Future<Output = Result<(), Error>>;
}

impl StackStatus for Receiver<Callback> {
    async fn stack_status(&mut self, target_status: Status) -> Result<(), Error> {
        loop {
            match self.recv().await {
                Ok(Some(callback)) => {
                    if let Callback::Networking(NetworkingEvent::StackStatus(status)) = callback
                        && status.result() == Ok(target_status)
                    {
                        return Ok(());
                    }
                }
                Ok(None) => {
                    return Err(Error::Io(io::Error::new(
                        ErrorKind::BrokenPipe,
                        "Callback channel is closed",
                    )));
                }
                Err(error) => {
                    return Err(Error::Io(io::Error::new(ErrorKind::BrokenPipe, error)));
                }
            }
        }
    }
}
