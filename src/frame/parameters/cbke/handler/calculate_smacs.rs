use le_stream::derive::FromLeStream;

use crate::ember::{SmacData, Status};
use crate::frame::Parameter;
use crate::resolve::Resolve;
use crate::Error;

const ID: u16 = 0x00A0;

/// A callback to indicate that the NCP has finished calculating the
/// Secure Message Authentication Codes (SMAC) for both the initiator and responder.
///
/// The associated link key is kept in temporary storage until the host tells the NCP
/// to store or discard the key via `emberClearTemporaryDataMaybeStoreLinkKey()`.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
    payload: Payload,
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Handler {
    /// The Result of the CBKE operation.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the status is not [`Status::Success`].
    pub fn result(&self) -> Result<Payload, Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|()| self.payload)
    }
}

/// The Result of the CBKE operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, FromLeStream)]
pub struct Payload {
    initiator_smac: SmacData,
    responder_smac: SmacData,
}

impl Payload {
    /// The calculated value of the initiator's SMAC
    #[must_use]
    pub const fn initiator_smac(&self) -> SmacData {
        self.initiator_smac
    }

    /// The calculated value of the responder's SMAC
    #[must_use]
    pub const fn responder_smac(&self) -> SmacData {
        self.responder_smac
    }
}
