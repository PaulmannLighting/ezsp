//! Shared ownership of an EZSP transport.

use std::num::NonZero;
use std::sync::Arc;

use le_stream::ToLeStream;
use tokio::sync::Mutex;

use crate::frame::Parameter;
use crate::parameters::configuration::version::Response;
use crate::{Connection, Error, Parameters, Transport};

impl<T> Transport for Arc<Mutex<T>>
where
    T: Transport,
{
    async fn connect(&mut self) -> Result<Response, Error> {
        self.lock().await.connect().await
    }

    async fn state(&self) -> Connection {
        self.lock().await.state().await
    }

    async fn negotiated_version(&self) -> Option<NonZero<u8>> {
        self.lock().await.negotiated_version().await
    }

    async fn send<U>(&mut self, command: U) -> Result<u16, Error>
    where
        U: Parameter + ToLeStream,
    {
        self.lock().await.send(command).await
    }

    async fn receive<U>(&mut self) -> Result<U, Error>
    where
        U: TryFrom<Parameters> + Send,
        <U as TryFrom<Parameters>>::Error: Into<Parameters> + Send,
    {
        self.lock().await.receive().await
    }
}
