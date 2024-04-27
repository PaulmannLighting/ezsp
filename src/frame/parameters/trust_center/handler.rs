pub mod trust_center_join;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    TrustCenterJoin(trust_center_join::Handler),
}

impl From<trust_center_join::Handler> for Handler {
    fn from(handler: trust_center_join::Handler) -> Self {
        Self::TrustCenterJoin(handler)
    }
}
