use crate::types::ByteSizedVec;
use crate::{Error, Transport};
use std::future::Future;

pub trait Utilities: Transport {
    /// Provides the customer a custom EZSP frame.
    /// On the NCP, these frames are only handled if the XNCP library is included.
    /// On the NCP side these frames are handled in the `emberXNcpIncomingCustomEzspMessageCallback()` callback function.
    fn custom_frame(
        &self,
        payload: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<ByteSizedVec<u8>, Error>> + Send;
}
