use std::num::NonZero;

use le_stream::ToLeStream;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

use crate::api::Message;
use crate::api::negotiate_version::NegotiateVersion;
use crate::frame::{Commands, Parameter, RespondsWith};
use crate::{Communicate, Error, Status, ezsp};

/// Cloneable handle to a connected EZSP transmitter actor.
///
/// [`Client::connect`](crate::Client::connect) creates this handle after a
/// successful version negotiation.
///
/// Each typed transaction is sent to the actor through a bounded channel and
/// completed through a one-shot response channel. Clones can be used by
/// independent tasks; the actor assigns sequence numbers and correlates their
/// responses.
#[derive(Clone, Debug)]
pub struct Connection {
    pub(crate) desired_version: NonZero<u8>,
    pub(crate) handle: Sender<Message>,
}

impl Communicate for Connection {
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

        match rx.await? {
            Ok(parameters) => match parameters.try_into() {
                Ok(command) => Ok(command),
                Err(error) => Err(Error::UnexpectedResponse(Box::new(error.into()))),
            },
            Err(error) => {
                if matches!(
                    error,
                    Error::Status(Status::Ezsp(Ok(ezsp::Status::Error(
                        ezsp::Error::VersionNotSet,
                    ))))
                ) {
                    self.handle.negotiate_version(self.desired_version).await?;
                }

                Err(error)
            }
        }
    }
}
