pub type ByteSizedVec<T> = heapless::Vec<T, { u8::MAX as usize }>;
pub type UintT = u16; // TODO: is this the correct type?

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum SourceRouteDiscoveryMode {
    Off = 0,
    On = 1,
    Reschedule = 2,
}

impl From<SourceRouteDiscoveryMode> for u8 {
    fn from(mode: SourceRouteDiscoveryMode) -> u8 {
        mode as u8
    }
}
