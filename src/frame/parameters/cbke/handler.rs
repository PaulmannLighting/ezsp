//! CBKE handlers.

use le_stream::{FromLeStream, ToLeStream};

pub use self::calculate_smacs::Handler as CalculateSmacs;
pub use self::calculate_smacs283k1::Handler as CalculateSmacs283k1;
pub use self::dsa_sign::Handler as DsaSign;
pub use self::dsa_verify::Handler as DsaVerify;
pub use self::generate_cbke_keys::Handler as GenerateCbkeKeys;
pub use self::generate_cbke_keys283k1::Handler as GenerateCbkeKeys283k1;
use crate::ember::SmacData;

mod calculate_smacs;
mod calculate_smacs283k1;
mod dsa_sign;
mod dsa_verify;
mod generate_cbke_keys;
mod generate_cbke_keys283k1;
crate::frame::parameters::parameter_enum!(
    Handler,
    CalculateSmacs,
    CalculateSmacs283k1,
    DsaSign,
    DsaVerify,
    GenerateCbkeKeys,
    GenerateCbkeKeys283k1
);

/// The Result of the CBKE operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
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
