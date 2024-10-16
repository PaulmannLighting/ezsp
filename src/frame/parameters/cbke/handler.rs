//! CBKE handler.

mod calculate_smacs;
mod calculate_smacs283k1;
mod dsa_sign;
mod dsa_verify;
mod generate_cbke_keys;
mod generate_cbke_keys283k1;

pub use calculate_smacs::Handler as CalculateSmacs;
pub use calculate_smacs283k1::Handler as CalculateSmacs283k1;
pub use dsa_sign::Handler as DsaSign;
pub use dsa_verify::Handler as DsaVerify;
pub use generate_cbke_keys::Handler as GenerateCbkeKeys;
pub use generate_cbke_keys283k1::Handler as GenerateCbkeKeys283k1;

/// Certificate-Based Key Exchange (CBKE) handlers.
#[allow(variant_size_differences, clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// Calculate Secure Message Authentication Codes (SMACs).
    CalculateSmacs(CalculateSmacs),
    /// Calculate Secure Message Authentication Codes (SMACs) for 283k1.
    CalculateSmacs283k1(CalculateSmacs283k1),
    /// Digital Signature Algorithm (DSA) sign.
    DsaSign(DsaSign),
    /// Digital Signature Algorithm (DSA) verify.
    DsaVerify(DsaVerify),
    /// Generate CBKE keys.
    GenerateCbkeKeys(GenerateCbkeKeys),
    /// Generate CBKE keys for 283k1.
    GenerateCbkeKeys283k1(GenerateCbkeKeys283k1),
}

impl From<CalculateSmacs> for Handler {
    fn from(handler: CalculateSmacs) -> Self {
        Self::CalculateSmacs(handler)
    }
}

impl From<CalculateSmacs283k1> for Handler {
    fn from(handler: CalculateSmacs283k1) -> Self {
        Self::CalculateSmacs283k1(handler)
    }
}

impl From<DsaSign> for Handler {
    fn from(handler: DsaSign) -> Self {
        Self::DsaSign(handler)
    }
}

impl From<DsaVerify> for Handler {
    fn from(handler: DsaVerify) -> Self {
        Self::DsaVerify(handler)
    }
}

impl From<GenerateCbkeKeys> for Handler {
    fn from(handler: GenerateCbkeKeys) -> Self {
        Self::GenerateCbkeKeys(handler)
    }
}

impl From<GenerateCbkeKeys283k1> for Handler {
    fn from(handler: GenerateCbkeKeys283k1) -> Self {
        Self::GenerateCbkeKeys283k1(handler)
    }
}
