//! Handlers for the trust center commands.

pub use self::trust_center_join::Handler as TrustCenterJoin;

mod trust_center_join;

/// The handler for the trust center command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// The handler for the trust center join command.
    TrustCenterJoin(TrustCenterJoin),
}
