use std::num::NonZero;
use std::sync::Arc;

use le_stream::ToLeStream;
use tokio::sync::mpsc::Sender;
use tokio::sync::{OwnedRwLockWriteGuard, RwLock, oneshot};

use crate::api::Message;
use crate::api::message::CommandGuard;
use crate::api::negotiate_version::NegotiateVersion;
use crate::frame::{Commands, Parameter, RespondsWith};
use crate::{Communicate, Error, ezsp};

/// Cloneable handle to a connected EZSP transmitter actor.
///
/// [`Client::connect`](crate::Client::connect) creates this handle after a
/// successful version negotiation.
///
/// Each typed transaction is sent to the actor through a bounded channel and
/// completed through a one-shot response channel. Owned handles and shared
/// references both implement [`Communicate`]. The actor assigns sequence
/// numbers and correlates responses across callers.
#[derive(Clone, Debug)]
pub struct Connection {
    pub(crate) desired_version: NonZero<u8>,
    pub(crate) handle: Sender<Message>,
    pub(crate) gate: Arc<RwLock<()>>,
}

/// Exclusive command session which waits for in-flight transactions and blocks
/// all clones until dropped. It does not stop autonomous firmware activity.
#[derive(Debug)]
pub struct MaintenanceConnection {
    connection: Connection,
    guard: Arc<OwnedRwLockWriteGuard<()>>,
}

impl Connection {
    /// Acquire exclusive access to this connection and all of its clones.
    ///
    /// Responses continue to flow through the transport actors. Callers must
    /// drain callbacks and ensure firmware quiescence independently.
    pub async fn maintenance(&self) -> MaintenanceConnection {
        let guard = self.gate.clone().write_owned().await;
        MaintenanceConnection {
            connection: self.clone(),
            guard: Arc::new(guard),
        }
    }

    async fn transact<T>(&self, command: T, guard: CommandGuard) -> Result<T::Response, Error>
    where
        T: Parameter + RespondsWith + ToLeStream + Into<Commands>,
    {
        let (response, rx) = oneshot::channel();

        self.handle
            .send(Message::Command {
                command: command.into(),
                response,
                guard: guard.clone(),
            })
            .await?;

        match rx.await? {
            Ok(parameters) => match parameters.try_into() {
                Ok(command) => Ok(command),
                Err(error) => Err(Error::UnexpectedResponse(Box::new(error.into()))),
            },
            Err(error) => {
                if error.is_ezsp_error(ezsp::Error::VersionNotSet) {
                    self.handle.negotiate_version(self.desired_version).await?;
                }

                Err(error)
            }
        }
    }
}

impl Communicate for Connection {
    async fn communicate<T>(&mut self, command: T) -> Result<T::Response, Error>
    where
        T: Parameter + RespondsWith + ToLeStream + Into<Commands>,
    {
        (&*self).communicate(command).await
    }
}

impl Communicate for &Connection {
    async fn communicate<T>(&mut self, command: T) -> Result<T::Response, Error>
    where
        T: Parameter + RespondsWith + ToLeStream + Into<Commands>,
    {
        let guard = Arc::new(self.gate.clone().read_owned().await);
        self.transact(command, CommandGuard::Read { _guard: guard })
            .await
    }
}

impl Communicate for MaintenanceConnection {
    async fn communicate<T>(&mut self, command: T) -> Result<T::Response, Error>
    where
        T: Parameter + RespondsWith + ToLeStream + Into<Commands>,
    {
        self.connection
            .transact(
                command,
                CommandGuard::Write {
                    _guard: self.guard.clone(),
                },
            )
            .await
    }
}
