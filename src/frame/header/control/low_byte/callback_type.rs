use num_derive::FromPrimitive;

#[derive(Debug, Clone, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum CallbackType {
    Reserved = 0b11,
    Asynchronous = 0b01,
    Synchronous = 0b10,
}

impl From<CallbackType> for u8 {
    fn from(callback_type: CallbackType) -> Self {
        callback_type as Self
    }
}
