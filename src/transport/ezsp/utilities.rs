use crate::ember::event;
use crate::ezsp::mfg_token::Id;
use crate::frame::parameters::utilities::{
    callback, custom_frame, debug_write, delay_test, echo, get_mfg_token, get_random_number,
    get_token, nop, set_mfg_token, set_timer, set_token,
};
use crate::frame::Handler;
use crate::types::ByteSizedVec;
use crate::{Command, Extended, Resolve};
use crate::{Error, Transport};
use std::future::Future;

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
    fn delay_test(&mut self, delay_millis: u16) -> impl Future<Output = Result<(), Error>> + Send;

    /// Variable length data from the Host is echoed back by the NCP.
    /// This command has no other effects and is designed for testing the link between the Host and NCP.
    fn echo(
        &mut self,
        data: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<ByteSizedVec<u8>, Error>> + Send;

    /// Retrieves a manufacturing token from the Flash Information Area of the NCP
    /// (except for `EZSP_STACK_CAL_DATA` which is managed by the stack).
    fn get_mfg_token(
        &mut self,
        token_id: Id,
    ) -> impl Future<Output = Result<ByteSizedVec<u8>, Error>> + Send;

    /// Returns a pseudorandom number.
    fn get_random_number(&mut self) -> impl Future<Output = Result<u16, Error>> + Send;

    /// Retrieves a token (8 bytes of non-volatile storage) from the Simulated EEPROM of the NCP.
    fn get_token(&mut self, token_id: u8) -> impl Future<Output = Result<[u8; 8], Error>> + Send;

    /// A command which does nothing.
    ///
    /// The Host can use this to set the sleep mode or to check the status of the NCP.
    fn nop(&mut self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sets a manufacturing token in the Customer Information Block (CIB) area of the NCP
    /// if that token currently unset (fully erased).
    ///
    /// Cannot be used with `EZSP_STACK_CAL_DATA`, `EZSP_STACK_CAL_FILTER`, `EZSP_MFG_ASH_CONFIG`,
    /// or `EZSP_MFG_CBKE_DATA` token.
    fn set_mfg_token(
        &mut self,
        token_id: Id,
        token: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sets a timer on the NCP. There are 2 independent timers available for use by the Host.
    ///
    /// A timer can be cancelled by setting time to 0 or units to `EMBER_EVENT_INACTIVE`.
    fn set_timer(
        &mut self,
        timer_id: u8,
        duration: event::Duration,
        repeat: bool,
    ) -> impl Future<Output = Result<(), Error>> + Send;

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

    async fn delay_test(&mut self, delay_millis: u16) -> Result<(), Error> {
        self.communicate::<_, delay_test::Response>(delay_test::Command::new(delay_millis))
            .await
            .map(drop)
    }

    async fn echo(&mut self, data: ByteSizedVec<u8>) -> Result<ByteSizedVec<u8>, Error> {
        self.communicate::<_, echo::Response>(echo::Command::new(data))
            .await
            .map(echo::Response::echo)
    }

    async fn get_mfg_token(&mut self, token_id: Id) -> Result<ByteSizedVec<u8>, Error> {
        self.communicate::<_, get_mfg_token::Response>(get_mfg_token::Command::new(token_id))
            .await
            .map(Into::into)
    }

    async fn get_random_number(&mut self) -> Result<u16, Error> {
        self.communicate::<_, get_random_number::Response>(get_random_number::Command)
            .await?
            .resolve()
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

    async fn set_mfg_token(&mut self, token_id: Id, token: ByteSizedVec<u8>) -> Result<(), Error> {
        self.communicate::<_, set_mfg_token::Response>(set_mfg_token::Command::new(token_id, token))
            .await?
            .resolve()
    }

    async fn set_timer(
        &mut self,
        timer_id: u8,
        duration: event::Duration,
        repeat: bool,
    ) -> Result<(), Error> {
        self.communicate::<_, set_timer::Response>(set_timer::Command::new(
            timer_id,
            duration.time(),
            duration.units(),
            repeat,
        ))
        .await?
        .resolve()
    }

    async fn set_token(&mut self, token_id: u8, token: [u8; 8]) -> Result<(), Error> {
        self.communicate::<_, set_token::Response>(set_token::Command::new(token_id, token))
            .await?
            .resolve()
    }
}
