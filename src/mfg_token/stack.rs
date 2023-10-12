use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Stack {
    CalData = 0x08,
    CalFilter = 0x0B,
}
