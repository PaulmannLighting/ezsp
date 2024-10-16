/// Frame Format Version.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FrameFormatVersion {
    /// Reserved bitflag.
    Reserved,
    /// Version 1.
    One,
    /// Version 0.
    Zero,
}
