pub mod trust_center_join;

/// The handler for the trust center command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// The handler for the trust center join command.
    TrustCenterJoin(trust_center_join::Handler),
}

impl From<trust_center_join::Handler> for Handler {
    fn from(handler: trust_center_join::Handler) -> Self {
        Self::TrustCenterJoin(handler)
    }
}
