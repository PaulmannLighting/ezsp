/// A stack-allocated vector with a maximum size of 255 elements.
pub type ByteSizedVec<T> = heapless::Vec<T, { u8::MAX as usize }>;

/// A type alias for a 16-bit unsigned integer.
///
/// See p. 129, `gpSinkTableGetNumberOfActiveEntries` of the `EZSP` specification.
pub type UintT = u16; // TODO: is this the correct type?

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
