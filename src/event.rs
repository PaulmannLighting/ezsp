use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Units {
    Inactive = 0x00,
    MsTime = 0x01,
    QsTime = 0x02,
    MinuteTime = 0x03,
}
