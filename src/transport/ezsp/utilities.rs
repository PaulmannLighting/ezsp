use std::future::Future;
use std::time::Duration;

use crate::frame::parameters::utilities::{
    callback, custom_frame, debug_write, delay_test, echo, get_token, nop, set_token,
};
use crate::frame::Handler;
use crate::types::ByteSizedVec;
use crate::{Command, Extended, Resolve};
use crate::{Error, Transport};

/// The `Utilities` trait provides an interface for the utilities.
pub trait Utilities {
    /// Allows the NCP to respond with a pending callback.
    ///
    /// The response to this command can be any of the callback responses.
    fn callback(&mut self) -> impl Future<Output = Result<Handler, Error>> + Send;

    /// Provides the customer a custom EZSP frame.
    /// On the NCP, these frames are only handled if the XNCP library is included.
    /// On the NCP side these frames are handled in the `emberXNcpIncomingCustomEzspMessageCallback()` callback function.
    fn custom_frame(
        &mut self,
        payload: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<ByteSizedVec<u8>, Error>> + Send;

    /// Sends a debug message from the Host to the Network Analyzer utility via the NCP.
    fn debug_write(
        &mut self,
        binary_message: bool,
        message: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Used to test that UART flow control is working correctly.
    fn delay_test(&mut self, delay: Duration) -> impl Future<Output = Result<(), Error>> + Send;

    /// Variable length data from the Host is echoed back by the NCP.
    /// This command has no other effects and is designed for testing the link between the Host and NCP.
    fn echo(
        &mut self,
        data: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<ByteSizedVec<u8>, Error>> + Send;

    /// Retrieves a token (8 bytes of non-volatile storage) from the Simulated EEPROM of the NCP.
    fn get_token(&mut self, token_id: u8) -> impl Future<Output = Result<[u8; 8], Error>> + Send;

    /// A command which does nothing.
    ///
    /// The Host can use this to set the sleep mode or to check the status of the NCP.
    fn nop(&mut self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sets a token (8 bytes of non-volatile storage) in the Simulated EEPROM of the NCP.
    fn set_token(
        &mut self,
        token_id: u8,
        token: [u8; 8],
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl<T> Utilities for T
where
    T: Transport,
{
    async fn callback(&mut self) -> Result<Handler, Error> {
        self.send::<Extended<Command>, _>(callback::Command).await?;
        todo!("Implement decoder for callback responses");
    }

    async fn custom_frame(&mut self, payload: ByteSizedVec<u8>) -> Result<ByteSizedVec<u8>, Error> {
        self.communicate::<_, custom_frame::Response>(custom_frame::Command::new(payload))
            .await?
            .into()
    }

    async fn debug_write(
        &mut self,
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

    async fn delay_test(&mut self, delay: Duration) -> Result<(), Error> {
        self.communicate::<_, delay_test::Response>(delay_test::Command::new(delay)?)
            .await
            .map(drop)
    }

    async fn echo(&mut self, data: ByteSizedVec<u8>) -> Result<ByteSizedVec<u8>, Error> {
        self.communicate::<_, echo::Response>(echo::Command::new(data))
            .await
            .map(echo::Response::echo)
    }

    async fn get_token(&mut self, token_id: u8) -> Result<[u8; 8], Error> {
        self.communicate::<_, get_token::Response>(get_token::Command::new(token_id))
            .await?
            .resolve()
    }

    async fn nop(&mut self) -> Result<(), Error> {
        self.communicate::<_, nop::Response>(nop::Command)
            .await
            .map(drop)
    }

    async fn set_token(&mut self, token_id: u8, token: [u8; 8]) -> Result<(), Error> {
        self.communicate::<_, set_token::Response>(set_token::Command::new(token_id, token))
            .await?
            .resolve()
    }
}
