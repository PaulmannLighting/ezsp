use num_derive::FromPrimitive;

/// Manufacturing token IDs pertaining to the manufacturer.
#[derive(Debug, Clone, Copy, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Mfg {
    /// Custom version (2 bytes).
    CustomVersion = 0x00,
    ///Manufacturing string (16 bytes).
    String = 0x01,
    /// Board name (16 bytes).
    BoardName = 0x02,
    /// Manufacturing ID (2 bytes).
    ManufId = 0x03,
    /// Radio configuration (2 bytes).
    PhyConfig = 0x04,
    /// Bootload AES key (16 bytes).
    BootloadAesKey = 0x05,
    /// ASH configuration (40 bytes).
    AshConfig = 0x06,
    /// EZSP storage (8 bytes).
    EzspStorage = 0x07,
    /// Certificate Based Key Exchange (CBKE) data (92 bytes).
    CbkeData = 0x09,
    /// Installation code (20 bytes).
    InstallationCode = 0x0A,
    /// Custom EUI64 MAC address (8 bytes).
    CustomEui64 = 0x0C,
    /// CTUNE value (2 byte).
    CTune = 0x0D,
}

impl From<Mfg> for u8 {
    fn from(manufacturing: Mfg) -> Self {
        manufacturing as Self
    }
}
