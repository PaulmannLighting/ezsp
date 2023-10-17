use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Flash {
    WriteInhibited = 0x46,
    VerifyFailed = 0x47,
    ProgFail = 0x4B,
    EraseFail = 0x4C,
}
