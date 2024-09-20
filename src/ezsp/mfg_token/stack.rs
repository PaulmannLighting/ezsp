use num_derive::FromPrimitive;

/// Manufacturing token IDs pertaining to the stack.
#[derive(Debug, Clone, Copy, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Stack {
    /// Radio calibration data (64 bytes).
    ///
    /// 4 bytes are stored for each of the 16 channels.
    /// This token is not stored in the Flash Information Area.
    /// It is updated by the stack each time a calibration is performed.
    CalData = 0x08,
    /// Radio channel filter calibration data (1 byte).
    ///
    /// This token is not stored in the Flash Information Area.
    /// It is updated by the stack each time a calibration is performed.
    CalFilter = 0x0B,
}

impl From<Stack> for u8 {
    fn from(stack: Stack) -> Self {
        stack as Self
    }
}
