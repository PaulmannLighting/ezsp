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

pub trait Header<T>:
    Clone + Copy + Debug + Eq + Hash + PartialEq + FromLeStream + ToLeStream + Send
where
    T: Copy + Clone + Debug + Eq + Hash + Into<u16> + PartialEq + Send,
{
    fn new(sequence: u8, low_byte: LowByte, id: T) -> Self;
    fn sequence(self) -> u8;
    fn low_byte(self) -> LowByte;
    fn high_byte(self) -> Option<HighByte>;
    fn id(self) -> T;
}
