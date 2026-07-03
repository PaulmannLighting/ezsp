//! Handlers for the trust center commands.

pub use self::trust_center_join::Handler as TrustCenterJoin;

mod trust_center_join;
crate::frame::parameters::parameter_enum!(Handler, TrustCenterJoin);
