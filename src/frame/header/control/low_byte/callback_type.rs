use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

#[derive(Debug, Clone, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum CallbackType {
    Reserved = 0b11,
    Asynchronous = 0b01,
    Synchronous = 0b10,
}

impl From<CallbackType> for u8 {
    fn from(callback_type: CallbackType) -> Self {
        callback_type
            .to_u8()
            .expect("CallbackType should always be convertible to u8.")
    }
}
