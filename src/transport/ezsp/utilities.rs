use crate::types::ByteSizedVec;
use crate::{Error, Transport};
use std::future::Future;
use std::time::Duration;

pub trait Utilities: Transport {
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
