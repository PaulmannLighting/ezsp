use le_stream::ToLeStream;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

use crate::frame::{Commands, Parameter, RespondsWith};
use crate::transceiver::Message;
use crate::{Communicate, Error};

/// Cloneable handle to a connected EZSP transmitter actor.
///
/// Each typed transaction is sent to the actor through a bounded channel and
/// completed through a one-shot response channel. Clones can be used by
/// independent tasks; the actor assigns sequence numbers and correlates their
/// responses.
#[derive(Clone, Debug)]
pub struct Connected {
    pub(crate) handle: Sender<Message>,
}

impl Communicate for Connected {
    async fn communicate<T>(&mut self, command: T) -> Result<T::Response, Error>
    where
        T: Parameter + RespondsWith + ToLeStream + Into<Commands>,
    {
        let (response, rx) = oneshot::channel();

        self.handle
            .send(Message::Command {
                command: command.into(),
                response,
            })
            .await?;

        match rx.await??.try_into() {
            Ok(command) => Ok(command),
            Err(error) => Err(Error::UnexpectedResponse(Box::new(error.into()))),
        }
    }
}
