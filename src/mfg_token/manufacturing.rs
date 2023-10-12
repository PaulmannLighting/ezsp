use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Manufacturing {
    CustomVersion = 0x00,
    String = 0x01,
    BoardName = 0x02,
    ManufId = 0x03,
    PhyConfig = 0x04,
    BootloadAesKey = 0x05,
    AshConfig = 0x06,
    EzspStorage = 0x07,
    CbkeData = 0x09,
    InstallationCode = 0x0A,
    CustomEui64 = 0x0C,
    CTune = 0x0D,
}
