use std::future::Future;

use crate::ember::constants::COUNTER_TYPE_COUNT;
use crate::ember::entropy::Source;
use crate::ember::{event, library, Eui64, NodeId};
use crate::ezsp::mfg_token::Id;
use crate::frame::parameters::utilities::{
    callback, custom_frame, debug_write, delay_test, echo, get_eui64, get_library_status,
    get_mfg_token, get_node_id, get_phy_interface_count, get_random_number, get_timer, get_token,
    get_true_random_entropy_source, get_xncp_info, nop, read_and_clear_counters, read_counters,
    set_mfg_token, set_timer, set_token,
};
use crate::frame::Handler;
use crate::types::ByteSizedVec;
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
    fn delay_test(&mut self, delay_millis: u16) -> impl Future<Output = Result<(), Error>> + Send;

    /// Variable length data from the Host is echoed back by the NCP.
    /// This command has no other effects and is designed for testing the link between the Host and NCP.
    fn echo(
        &mut self,
        data: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<ByteSizedVec<u8>, Error>> + Send;

    /// Returns the EUI64 ID of the local node.
    fn get_eui64(&mut self) -> impl Future<Output = Result<Eui64, Error>> + Send;

    /// This retrieves the status of the passed library ID to determine
    /// if it is compiled into the stack.
    fn get_library_status(
        &mut self,
        library_id: library::Id,
    ) -> impl Future<Output = Result<library::Status, Error>> + Send;

    /// Retrieves a manufacturing token from the Flash Information Area of the NCP
    /// (except for `EZSP_STACK_CAL_DATA` which is managed by the stack).
    fn get_mfg_token(
        &mut self,
        token_id: Id,
    ) -> impl Future<Output = Result<ByteSizedVec<u8>, Error>> + Send;

    /// Returns the 16-bit node ID of the local node.
    fn get_node_id(&mut self) -> impl Future<Output = Result<NodeId, Error>> + Send;

    /// Returns number of phy interfaces present.
    fn get_phy_interface_count(&mut self) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Returns a pseudorandom number.
    fn get_random_number(&mut self) -> impl Future<Output = Result<u16, Error>> + Send;

    /// Gets information about a timer.
    ///
    /// The Host can use this command to find out how much longer it will be before a previously
    /// set timer will generate a callback.
    fn get_timer(
        &mut self,
        timer_id: u8,
    ) -> impl Future<Output = Result<get_timer::Response, Error>> + Send;

    /// Retrieves a token (8 bytes of non-volatile storage) from the Simulated EEPROM of the NCP.
    fn get_token(&mut self, token_id: u8) -> impl Future<Output = Result<[u8; 8], Error>> + Send;

    /// Returns the entropy source used for true random number generation.
    fn get_true_random_entropy_source(
        &mut self,
    ) -> impl Future<Output = Result<Source, Error>> + Send;

    /// Allows the HOST to know whether the NCP is running the XNCP library.
    ///
    /// If so, the response contains also the manufacturer ID and the version number of the
    /// XNCP application that is running on the NCP.
    fn get_xncp_info(
        &mut self,
    ) -> impl Future<Output = Result<get_xncp_info::Payload, Error>> + Send;

    /// A command which does nothing.
    ///
    /// The Host can use this to set the sleep mode or to check the status of the NCP.
    fn nop(&mut self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Retrieves and clears Ember counters.
    ///
    /// See the [`crate::ember::counter::Type`] enumeration for the counter types.
    fn read_and_clear_counters(
        &mut self,
    ) -> impl Future<Output = Result<[u16; COUNTER_TYPE_COUNT], Error>> + Send;

    /// Retrieves Ember counters.
    ///
    /// See the [`crate::ember::counter::Type`] enumeration for the counter types.
    fn read_counters(
        &mut self,
    ) -> impl Future<Output = Result<[u16; COUNTER_TYPE_COUNT], Error>> + Send;

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
        self.send(callback::Command).await?;
        todo!("Implement decoder for callback responses");
    }

    async fn custom_frame(&mut self, payload: ByteSizedVec<u8>) -> Result<ByteSizedVec<u8>, Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, custom_frame::Response>(custom_frame::Command::new(payload))
            .await?
            .try_into()
    }

    async fn debug_write(
        &mut self,
        binary_message: bool,
        message: ByteSizedVec<u8>,
    ) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, debug_write::Response>(debug_write::Command::new(
            binary_message,
            message,
        ))
        .await?
        .try_into()
    }

    async fn delay_test(&mut self, delay_millis: u16) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, delay_test::Response>(delay_test::Command::new(delay_millis))
            .await
            .map(drop)
    }

    async fn echo(&mut self, data: ByteSizedVec<u8>) -> Result<ByteSizedVec<u8>, Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, echo::Response>(echo::Command::new(data))
            .await
            .map(echo::Response::echo)
    }

    async fn get_eui64(&mut self) -> Result<Eui64, Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, get_eui64::Response>(get_eui64::Command)
            .await
            .map(Into::into)
    }

    async fn get_library_status(
        &mut self,
        library_id: library::Id,
    ) -> Result<library::Status, Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, get_library_status::Response>(get_library_status::Command::new(
            library_id,
        ))
        .await
        .map(Into::into)
    }

    async fn get_mfg_token(&mut self, token_id: Id) -> Result<ByteSizedVec<u8>, Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, get_mfg_token::Response>(get_mfg_token::Command::new(token_id))
            .await
            .map(Into::into)
    }

    async fn get_node_id(&mut self) -> Result<NodeId, Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, get_node_id::Response>(get_node_id::Command)
            .await
            .map(Into::into)
    }

    async fn get_phy_interface_count(&mut self) -> Result<u8, Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, get_phy_interface_count::Response>(get_phy_interface_count::Command)
            .await
            .map(Into::into)
    }

    async fn get_random_number(&mut self) -> Result<u16, Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, get_random_number::Response>(get_random_number::Command)
            .await?
            .try_into()
    }

    async fn get_timer(&mut self, timer_id: u8) -> Result<get_timer::Response, Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, get_timer::Response>(get_timer::Command::new(timer_id))
            .await
    }

    async fn get_token(&mut self, token_id: u8) -> Result<[u8; 8], Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, get_token::Response>(get_token::Command::new(token_id))
            .await?
            .try_into()
    }

    async fn get_true_random_entropy_source(&mut self) -> Result<Source, Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, get_true_random_entropy_source::Response>(
            get_true_random_entropy_source::Command,
        )
        .await?
        .try_into()
    }

    async fn get_xncp_info(&mut self) -> Result<get_xncp_info::Payload, Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, get_xncp_info::Response>(get_xncp_info::Command)
            .await?
            .try_into()
    }

    async fn nop(&mut self) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, nop::Response>(nop::Command)
            .await
            .map(drop)
    }

    async fn read_and_clear_counters(&mut self) -> Result<[u16; COUNTER_TYPE_COUNT], Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, read_and_clear_counters::Response>(read_and_clear_counters::Command)
            .await
            .map(Into::into)
    }

    async fn read_counters(&mut self) -> Result<[u16; COUNTER_TYPE_COUNT], Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, read_counters::Response>(read_counters::Command)
            .await
            .map(Into::into)
    }

    async fn set_mfg_token(&mut self, token_id: Id, token: ByteSizedVec<u8>) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, set_mfg_token::Response>(set_mfg_token::Command::new(token_id, token))
            .await?
            .try_into()
    }

    async fn set_timer(
        &mut self,
        timer_id: u8,
        duration: event::Duration,
        repeat: bool,
    ) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, set_timer::Response>(set_timer::Command::new(
            timer_id,
            duration.time(),
            duration.units(),
            repeat,
        ))
        .await?
        .try_into()
    }

    async fn set_token(&mut self, token_id: u8, token: [u8; 8]) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, set_token::Response>(set_token::Command::new(token_id, token))
            .await?
            .try_into()
    }
}
