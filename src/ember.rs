pub mod adc;
pub mod application;
pub mod beacon;
pub mod eeprom;
pub mod entropy;
pub mod err;
pub mod event;
pub mod join_method;
pub mod mac;
pub mod network;
pub mod node;
pub mod phy;
pub mod serial;
pub mod sim_eeprom;
pub mod zigbee;

use anyhow::anyhow;
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Status {
    Success = 0x00,
    ErrFatal = 0x01,
    BadArgument = 0x02,
    Eeprom(eeprom::Eeprom),
    NoBuffers = 0x18,
    Serial(serial::Serial),
    Mac(mac::Mac),
    SimEeprom(sim_eeprom::SimEeprom),
    Err(err::Err),
    DeliveryFailed = 0x66,
    BindingIndexOutOfRange = 0x69,
    AddressTableIndexOutOfRange = 0x6A,
    InvalidBindingIndex = 0x6C,
    InvalidCall = 0x70,
    CostNotKnown = 0x71,
    MaxMessageLimitReached = 0x72,
    MessageTooLong = 0x74,
    BindingIsActive = 0x75,
    AddressTableEntryIsActive = 0x76,
    Adc(adc::Adc),
    SleepInterrupted = 0x85,
    Phy(phy::Phy),
    NetworkUp = 0x90,
    NetworkDown = 0x91,
    NotJoined = 0x93,
    JoinFailed = 0x94,
    InvalidSecurityLevel = 0x95,
    MoveFailed = 0x96,
    CannotJoinAsRouter = 0x98,
    NodeIdChanged = 0x99,
    PanIdChanged = 0x9A,
    NetworkOpened = 0x9C,
    NetworkClosed = 0x9D,
    NoBeacons = 0xAB,
    ReceivedKeyInTheClear = 0xAC,
    NoNetworkKeyReceived = 0xAD,
    NoLinkKeyReceived = 0xAE,
    PreconfiguredKeyRequired = 0xAF,
    NetworkBusy = 0xA1,
    InvalidEndpoint = 0xA3,
    BindingHasChanged = 0xA4,
    InsufficientRandomData = 0xA5,
    ApsEncryptionError = 0xA6,
    SecurityStateNotSet = 0xA8,
    SourceRouteFailure = 0xA9,
    ManyToOneRouteFailure = 0xAA,
    StackAndHardwareMismatch = 0xB0,
    IndexOutOfRange = 0xB1,
    KeyTableInvalidAddress = 0xB3,
    TableFull = 0xB4,
    LibraryNotPresent = 0xB5,
    TableEntryErased = 0xB6,
    SecurityConfigurationInvalid = 0xB7,
    TooSoonForSwitchKey = 0xB8,
    OperationInProgress = 0xBA,
    KeyNotAuthorized = 0xBB,
    SecurityDataInvalid = 0xBD,
    Application(application::Application),
}

impl FromPrimitive for Status {
    fn from_i64(n: i64) -> Option<Self> {
        u8::try_from(n).ok().and_then(Self::from_u8)
    }

    fn from_u8(n: u8) -> Option<Self> {
        match n {
            0x00 => Some(Self::Success),
            0x01 => Some(Self::ErrFatal),
            0x02 => Some(Self::BadArgument),
            0x04..=0x07 => eeprom::Eeprom::from_u8(n).map(Self::Eeprom),
            0x18 => Some(Self::NoBuffers),
            0x20..=0x27 => serial::Serial::from_u8(n).map(Self::Serial),
            0x31..=0x42 => mac::Mac::from_u8(n).map(Self::Mac),
            0x43..=0x45 | 0x48..=0x4A => sim_eeprom::SimEeprom::from_u8(n).map(Self::SimEeprom),
            0x46..=0x4C | 0x58..=0x5A => err::Err::from_u8(n).map(Self::Err),
            0x66 => Some(Self::DeliveryFailed),
            0x69 => Some(Self::BindingIndexOutOfRange),
            0x6A => Some(Self::AddressTableIndexOutOfRange),
            0x6C => Some(Self::InvalidBindingIndex),
            0x70 => Some(Self::InvalidCall),
            0x71 => Some(Self::CostNotKnown),
            0x72 => Some(Self::MaxMessageLimitReached),
            0x74 => Some(Self::MessageTooLong),
            0x75 => Some(Self::BindingIsActive),
            0x76 => Some(Self::AddressTableEntryIsActive),
            0x80..=0x84 => adc::Adc::from_u8(n).map(Self::Adc),
            0x85 => Some(Self::SleepInterrupted),
            0x88..=0x8F => phy::Phy::from_u8(n).map(Self::Phy),
            0x90 => Some(Self::NetworkUp),
            0x91 => Some(Self::NetworkDown),
            0x93 => Some(Self::NotJoined),
            0x94 => Some(Self::JoinFailed),
            0x95 => Some(Self::InvalidSecurityLevel),
            0x96 => Some(Self::MoveFailed),
            0x98 => Some(Self::CannotJoinAsRouter),
            0x99 => Some(Self::NodeIdChanged),
            0x9A => Some(Self::PanIdChanged),
            0x9C => Some(Self::NetworkOpened),
            0x9D => Some(Self::NetworkClosed),
            0xAB => Some(Self::NoBeacons),
            0xAC => Some(Self::ReceivedKeyInTheClear),
            0xAD => Some(Self::NoNetworkKeyReceived),
            0xAE => Some(Self::NoLinkKeyReceived),
            0xAF => Some(Self::PreconfiguredKeyRequired),
            0xA1 => Some(Self::NetworkBusy),
            0xA3 => Some(Self::InvalidEndpoint),
            0xA4 => Some(Self::BindingHasChanged),
            0xA5 => Some(Self::InsufficientRandomData),
            0xA6 => Some(Self::ApsEncryptionError),
            0xA8 => Some(Self::SecurityStateNotSet),
            0xA9 => Some(Self::SourceRouteFailure),
            0xAA => Some(Self::ManyToOneRouteFailure),
            0xB0 => Some(Self::StackAndHardwareMismatch),
            0xB1 => Some(Self::IndexOutOfRange),
            0xB3 => Some(Self::KeyTableInvalidAddress),
            0xB4 => Some(Self::TableFull),
            0xB5 => Some(Self::LibraryNotPresent),
            0xB6 => Some(Self::TableEntryErased),
            0xB7 => Some(Self::SecurityConfigurationInvalid),
            0xB8 => Some(Self::TooSoonForSwitchKey),
            0xBA => Some(Self::OperationInProgress),
            0xBB => Some(Self::KeyNotAuthorized),
            0xBD => Some(Self::SecurityDataInvalid),
            0xF0..=0xFF => application::Application::from_u8(n).map(Self::Application),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        u8::try_from(n).ok().and_then(Self::from_u8)
    }
}

impl ToPrimitive for Status {
    fn to_i64(&self) -> Option<i64> {
        self.to_u8().map(i64::from)
    }

    fn to_u8(&self) -> Option<u8> {
        match self {
            Self::Success => Some(0x00),
            Self::ErrFatal => Some(0x01),
            Self::BadArgument => Some(0x02),
            Self::Eeprom(eeprom) => eeprom.to_u8(),
            Self::NoBuffers => Some(0x18),
            Self::Serial(serial) => serial.to_u8(),
            Self::Mac(mac) => mac.to_u8(),
            Self::SimEeprom(sim_eeprom) => sim_eeprom.to_u8(),
            Self::Err(err) => err.to_u8(),
            Self::DeliveryFailed => Some(0x66),
            Self::BindingIndexOutOfRange => Some(0x69),
            Self::AddressTableIndexOutOfRange => Some(0x6A),
            Self::InvalidBindingIndex => Some(0x6C),
            Self::InvalidCall => Some(0x70),
            Self::CostNotKnown => Some(0x71),
            Self::MaxMessageLimitReached => Some(0x72),
            Self::MessageTooLong => Some(0x74),
            Self::BindingIsActive => Some(0x75),
            Self::AddressTableEntryIsActive => Some(0x76),
            Self::Adc(adc) => adc.to_u8(),
            Self::SleepInterrupted => Some(0x85),
            Self::Phy(phy) => phy.to_u8(),
            Self::NetworkUp => Some(0x90),
            Self::NetworkDown => Some(0x91),
            Self::NotJoined => Some(0x93),
            Self::JoinFailed => Some(0x94),
            Self::InvalidSecurityLevel => Some(0x95),
            Self::MoveFailed => Some(0x96),
            Self::CannotJoinAsRouter => Some(0x98),
            Self::NodeIdChanged => Some(0x99),
            Self::PanIdChanged => Some(0x9A),
            Self::NetworkOpened => Some(0x9C),
            Self::NetworkClosed => Some(0x9D),
            Self::NoBeacons => Some(0xAB),
            Self::ReceivedKeyInTheClear => Some(0xAC),
            Self::NoNetworkKeyReceived => Some(0xAD),
            Self::NoLinkKeyReceived => Some(0xAE),
            Self::PreconfiguredKeyRequired => Some(0xAF),
            Self::NetworkBusy => Some(0xA1),
            Self::InvalidEndpoint => Some(0xA3),
            Self::BindingHasChanged => Some(0xA4),
            Self::InsufficientRandomData => Some(0xA5),
            Self::ApsEncryptionError => Some(0xA6),
            Self::SecurityStateNotSet => Some(0xA8),
            Self::SourceRouteFailure => Some(0xA9),
            Self::ManyToOneRouteFailure => Some(0xAA),
            Self::StackAndHardwareMismatch => Some(0xB0),
            Self::IndexOutOfRange => Some(0xB1),
            Self::KeyTableInvalidAddress => Some(0xB3),
            Self::TableFull => Some(0xB4),
            Self::LibraryNotPresent => Some(0xB5),
            Self::TableEntryErased => Some(0xB6),
            Self::SecurityConfigurationInvalid => Some(0xB7),
            Self::TooSoonForSwitchKey => Some(0xB8),
            Self::OperationInProgress => Some(0xBA),
            Self::KeyNotAuthorized => Some(0xBB),
            Self::SecurityDataInvalid => Some(0xBD),
            Self::Application(application) => application.to_u8(),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        self.to_u8().map(u64::from)
    }
}

impl TryFrom<u8> for Status {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or_else(|| anyhow!("invalid EmberStatus: {value:#04X}"))
    }
}

impl From<Status> for u8 {
    fn from(status: Status) -> Self {
        status.to_u8().expect("could not convert EmberStatus to u8")
    }
}
