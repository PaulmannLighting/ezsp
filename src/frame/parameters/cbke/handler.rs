//! CBKE handlers.

use le_stream::FromLeStream;

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

/// Certificate-Based Key Exchange (CBKE) handlers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// Calculate Secure Message Authentication Codes (SMACs).
    CalculateSmacs(CalculateSmacs),
    /// Calculate Secure Message Authentication Codes (SMACs) for 283k1.
    CalculateSmacs283k1(CalculateSmacs283k1),
    /// Digital Signature Algorithm (DSA) sign.
    DsaSign(Box<DsaSign>),
    /// Digital Signature Algorithm (DSA) verify.
    DsaVerify(DsaVerify),
    /// Generate CBKE keys.
    GenerateCbkeKeys(GenerateCbkeKeys),
    /// Generate CBKE keys for 283k1.
    GenerateCbkeKeys283k1(GenerateCbkeKeys283k1),
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
