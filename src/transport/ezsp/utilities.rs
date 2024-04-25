use std::future::Future;
use std::time::Duration;

use crate::error::Resolve;
use crate::frame::parameters::utilities::{custom_frame, debug_write, delay_test, echo};
use crate::types::ByteSizedVec;
use crate::{Error, Transport};

pub trait Utilities {
    /// Provides the customer a custom EZSP frame.
    /// On the NCP, these frames are only handled if the XNCP library is included.
    /// On the NCP side these frames are handled in the `emberXNcpIncomingCustomEzspMessageCallback()` callback function.
    fn custom_frame(
        &self,
        payload: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<ByteSizedVec<u8>, Error>> + Send;

    /// Sends a debug message from the Host to the Network Analyzer utility via the NCP.
    fn debug_write(
        &self,
        binary_message: bool,
        message: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Used to test that UART flow control is working correctly.
    fn delay_test(&self, delay: Duration) -> impl Future<Output = Result<(), Error>> + Send;

    /// Variable length data from the Host is echoed back by the NCP.
    /// This command has no other effects and is designed for testing the link between the Host and NCP.
    fn echo(
        &self,
        data: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<ByteSizedVec<u8>, Error>> + Send;
}

impl<T> Utilities for T
where
    T: Transport,
{
    async fn custom_frame(&self, payload: ByteSizedVec<u8>) -> Result<ByteSizedVec<u8>, Error> {
        self.communicate::<_, custom_frame::Response>(custom_frame::Command::new(payload))
            .await?
            .into()
    }

    async fn debug_write(
        &self,
        binary_message: bool,
        message: ByteSizedVec<u8>,
    ) -> Result<(), Error> {
        self.communicate::<_, debug_write::Response>(debug_write::Command::new(
            binary_message,
            message,
        ))
        .await?
        .status()
        .resolve()
    }

    async fn delay_test(&self, delay: Duration) -> Result<(), Error> {
        self.communicate::<_, delay_test::Response>(delay_test::Command::new(delay)?)
            .await
            .map(drop)
    }

    async fn echo(&self, data: ByteSizedVec<u8>) -> Result<ByteSizedVec<u8>, Error> {
        self.communicate::<_, echo::Response>(echo::Command::new(data))
            .await
            .map(echo::Response::echo)
    }
}
