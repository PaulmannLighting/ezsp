pub mod adc;
pub mod application;
pub mod eeprom;
pub mod err;
pub mod mac;
pub mod phy;
pub mod serial;
pub mod sim_eeprom;

use anyhow::anyhow;
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum EmberStatus {
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

impl TryFrom<u8> for EmberStatus {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Success),
            0x01 => Ok(Self::ErrFatal),
            0x02 => Ok(Self::BadArgument),
            0x04..=0x07 => eeprom::Eeprom::from_u8(value)
                .ok_or_else(|| anyhow!("invalid EEPROM value: {value}"))
                .map(Self::Eeprom),
            0x18 => Ok(Self::NoBuffers),
            0x20..=0x27 => serial::Serial::from_u8(value)
                .ok_or_else(|| anyhow!("invalid SERIAL value: {value}"))
                .map(Self::Serial),
            0x31..=0x42 => mac::Mac::from_u8(value)
                .ok_or_else(|| anyhow!("invalid MAC value: {value}"))
                .map(Self::Mac),
            0x43..=0x45 | 0x48..=0x4A => sim_eeprom::SimEeprom::from_u8(value)
                .ok_or_else(|| anyhow!("invalid SIM_EEPROM value: {value}"))
                .map(Self::SimEeprom),
            0x46..=0x4C | 0x58..=0x5A => err::Err::from_u8(value)
                .ok_or_else(|| anyhow!("invalid ERR value: {value}"))
                .map(Self::Err),
            0x66 => Ok(Self::DeliveryFailed),
            0x69 => Ok(Self::BindingIndexOutOfRange),
            0x6A => Ok(Self::AddressTableIndexOutOfRange),
            0x6C => Ok(Self::InvalidBindingIndex),
            0x70 => Ok(Self::InvalidCall),
            0x71 => Ok(Self::CostNotKnown),
            0x72 => Ok(Self::MaxMessageLimitReached),
            0x74 => Ok(Self::MessageTooLong),
            0x75 => Ok(Self::BindingIsActive),
            0x76 => Ok(Self::AddressTableEntryIsActive),
            0x80..=0x84 => adc::Adc::from_u8(value)
                .ok_or_else(|| anyhow!("invalid ADC value: {value}"))
                .map(Self::Adc),
            0x85 => Ok(Self::SleepInterrupted),
            0x88..=0x8F => phy::Phy::from_u8(value)
                .ok_or_else(|| anyhow!("invalid PHY value: {value}"))
                .map(Self::Phy),
            0x90 => Ok(Self::NetworkUp),
            0x91 => Ok(Self::NetworkDown),
            0x93 => Ok(Self::NotJoined),
            0x94 => Ok(Self::JoinFailed),
            0x95 => Ok(Self::InvalidSecurityLevel),
            0x96 => Ok(Self::MoveFailed),
            0x98 => Ok(Self::CannotJoinAsRouter),
            0x99 => Ok(Self::NodeIdChanged),
            0x9A => Ok(Self::PanIdChanged),
            0x9C => Ok(Self::NetworkOpened),
            0x9D => Ok(Self::NetworkClosed),
            0xAB => Ok(Self::NoBeacons),
            0xAC => Ok(Self::ReceivedKeyInTheClear),
            0xAD => Ok(Self::NoNetworkKeyReceived),
            0xAE => Ok(Self::NoLinkKeyReceived),
            0xAF => Ok(Self::PreconfiguredKeyRequired),
            0xA1 => Ok(Self::NetworkBusy),
            0xA3 => Ok(Self::InvalidEndpoint),
            0xA4 => Ok(Self::BindingHasChanged),
            0xA5 => Ok(Self::InsufficientRandomData),
            0xA6 => Ok(Self::ApsEncryptionError),
            0xA8 => Ok(Self::SecurityStateNotSet),
            0xA9 => Ok(Self::SourceRouteFailure),
            0xAA => Ok(Self::ManyToOneRouteFailure),
            0xB0 => Ok(Self::StackAndHardwareMismatch),
            0xB1 => Ok(Self::IndexOutOfRange),
            0xB3 => Ok(Self::KeyTableInvalidAddress),
            0xB4 => Ok(Self::TableFull),
            0xB5 => Ok(Self::LibraryNotPresent),
            0xB6 => Ok(Self::TableEntryErased),
            0xB7 => Ok(Self::SecurityConfigurationInvalid),
            0xB8 => Ok(Self::TooSoonForSwitchKey),
            0xBA => Ok(Self::OperationInProgress),
            0xBB => Ok(Self::KeyNotAuthorized),
            0xBD => Ok(Self::SecurityDataInvalid),
            0xF0..=0xFF => application::Application::from_u8(value)
                .ok_or_else(|| anyhow!("invalid PHY value: {value}"))
                .map(Self::Application),
            n => Err(anyhow!("invalid EmberStatus: {n:#04X}")),
        }
    }
}

#[allow(clippy::fallible_impl_from, clippy::unwrap_used)]
impl From<EmberStatus> for u8 {
    fn from(ember_status: EmberStatus) -> Self {
        match ember_status {
            EmberStatus::Success => 0x00,
            EmberStatus::ErrFatal => 0x01,
            EmberStatus::BadArgument => 0x02,
            EmberStatus::Eeprom(eeprom) => eeprom.to_u8().unwrap(),
            EmberStatus::NoBuffers => 0x18,
            EmberStatus::Serial(serial) => serial.to_u8().unwrap(),
            EmberStatus::Mac(mac) => mac.to_u8().unwrap(),
            EmberStatus::SimEeprom(sim_eeprom) => sim_eeprom.to_u8().unwrap(),
            EmberStatus::Err(err) => err.to_u8().unwrap(),
            EmberStatus::DeliveryFailed => 0x66,
            EmberStatus::BindingIndexOutOfRange => 0x69,
            EmberStatus::AddressTableIndexOutOfRange => 0x6A,
            EmberStatus::InvalidBindingIndex => 0x6C,
            EmberStatus::InvalidCall => 0x70,
            EmberStatus::CostNotKnown => 0x71,
            EmberStatus::MaxMessageLimitReached => 0x72,
            EmberStatus::MessageTooLong => 0x74,
            EmberStatus::BindingIsActive => 0x75,
            EmberStatus::AddressTableEntryIsActive => 0x76,
            EmberStatus::Adc(adc) => adc.to_u8().unwrap(),
            EmberStatus::SleepInterrupted => 0x85,
            EmberStatus::Phy(phy) => phy.to_u8().unwrap(),
            EmberStatus::NetworkUp => 0x90,
            EmberStatus::NetworkDown => 0x91,
            EmberStatus::NotJoined => 0x93,
            EmberStatus::JoinFailed => 0x94,
            EmberStatus::InvalidSecurityLevel => 0x95,
            EmberStatus::MoveFailed => 0x96,
            EmberStatus::CannotJoinAsRouter => 0x98,
            EmberStatus::NodeIdChanged => 0x99,
            EmberStatus::PanIdChanged => 0x9A,
            EmberStatus::NetworkOpened => 0x9C,
            EmberStatus::NetworkClosed => 0x9D,
            EmberStatus::NoBeacons => 0xAB,
            EmberStatus::ReceivedKeyInTheClear => 0xAC,
            EmberStatus::NoNetworkKeyReceived => 0xAD,
            EmberStatus::NoLinkKeyReceived => 0xAE,
            EmberStatus::PreconfiguredKeyRequired => 0xAF,
            EmberStatus::NetworkBusy => 0xA1,
            EmberStatus::InvalidEndpoint => 0xA3,
            EmberStatus::BindingHasChanged => 0xA4,
            EmberStatus::InsufficientRandomData => 0xA5,
            EmberStatus::ApsEncryptionError => 0xA6,
            EmberStatus::SecurityStateNotSet => 0xA8,
            EmberStatus::SourceRouteFailure => 0xA9,
            EmberStatus::ManyToOneRouteFailure => 0xAA,
            EmberStatus::StackAndHardwareMismatch => 0xB0,
            EmberStatus::IndexOutOfRange => 0xB1,
            EmberStatus::KeyTableInvalidAddress => 0xB3,
            EmberStatus::TableFull => 0xB4,
            EmberStatus::LibraryNotPresent => 0xB5,
            EmberStatus::TableEntryErased => 0xB6,
            EmberStatus::SecurityConfigurationInvalid => 0xB7,
            EmberStatus::TooSoonForSwitchKey => 0xB8,
            EmberStatus::OperationInProgress => 0xBA,
            EmberStatus::KeyNotAuthorized => 0xBB,
            EmberStatus::SecurityDataInvalid => 0xBD,
            EmberStatus::Application(application) => application.to_u8().unwrap(),
        }
    }
}
