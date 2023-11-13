use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum KeyStructBitmask {
    HasSequenceNumber = 0x00001,
    HasOutgoingFrameCounter = 0x0002,
    HasIncomingFrameCounter = 0x0004,
    HasPartnerEui64 = 0x0008,
}
