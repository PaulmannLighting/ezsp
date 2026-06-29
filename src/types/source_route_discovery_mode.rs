/// Discovery mode for source routes.
///
/// See p. 88 `setSourceRouteDiscoveryMode` of the `EZSP` specification.
#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum SourceRouteDiscoveryMode {
    /// Source route discovery is off.
    Off = 0,

    /// Source route discovery is on.
    On = 1,

    /// Source route discovery is set to reschedule.
    Reschedule = 2,
}

impl From<SourceRouteDiscoveryMode> for u8 {
    fn from(mode: SourceRouteDiscoveryMode) -> Self {
        mode as Self
    }
}
