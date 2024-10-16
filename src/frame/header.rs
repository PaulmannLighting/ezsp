mod extended;
mod high_byte;
mod legacy;
mod low_byte;

pub use extended::Extended;
pub use high_byte::{FrameFormatVersion, HighByte};
use le_stream::{FromLeStream, ToLeStream};
pub use legacy::Legacy;
pub use low_byte::{CallbackType, Command, LowByte, Response, SleepMode};
use std::fmt::Debug;
use std::hash::Hash;

/// A trait to represent the header of a frame.
pub trait Header<T>:
    Clone + Copy + Debug + Eq + Hash + PartialEq + FromLeStream + ToLeStream + Send
where
    T: Copy + Clone + Debug + Eq + Hash + Into<u16> + PartialEq + Send,
{
    /// Create a new header.
    fn new(sequence: u8, low_byte: LowByte, id: T) -> Self;

    /// Return the sequence number.
    fn sequence(self) -> u8;

    /// Return the low byte.
    fn low_byte(self) -> LowByte;

    /// Return the high byte.
    ///
    /// This method returns `None` for legacy frames.
    fn high_byte(self) -> Option<HighByte>;

    /// Return the frame ID.
    fn id(self) -> T;
}
