//! Handlers for the trust center commands.

mod trust_center_join;

pub use trust_center_join::Handler as TrustCenterJoin;

/// The handler for the trust center command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// The handler for the trust center join command.
    TrustCenterJoin(TrustCenterJoin),
}
