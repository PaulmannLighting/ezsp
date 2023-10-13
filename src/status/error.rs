use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Error {
    VersionNotSet = 0x30,
    InvalidFrameId = 0x31,
    WrongDirection = 0x32,
    Truncated = 0x33,
    Overflow = 0x34,
    OutOfMemory = 0x35,
    InvalidValue = 0x36,
    InvalidId = 0x37,
    InvalidCall = 0x38,
    NoResponse = 0x39,
    CommandTooLong = 0x40,
    QueueFull = 0x41,
    CommandFiltered = 0x42,
    SecurityKeyAlreadySet = 0x43,
    SecurityTypeInvalid = 0x44,
    SecurityParametersInvalid = 0x45,
    SecurityParametersAlreadySet = 0x46,
    SecurityKeyNotSet = 0x47,
    SecurityParametersNotSet = 0x48,
    UnsupportedControl = 0x49,
    UnsecureFrame = 0x4A,
}
