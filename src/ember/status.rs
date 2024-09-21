use std::fmt::{Debug, Display, Formatter};

use num_traits::FromPrimitive;

use crate::Error;
use crate::Resolve;
pub use adc::Adc;
pub use application::Application;
pub use eeprom::Eeprom;
pub use err::{Bootloader, Err, Flash};
pub use mac::Mac;
pub use phy::Phy;
pub use serial::Serial;
pub use sim_eeprom::SimEeprom;
use values::Values;

mod adc;
mod application;
mod eeprom;
mod err;
mod mac;
mod phy;
mod serial;
mod sim_eeprom;
mod values;

/// Ember status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Status {
    /// The generic 'no error' message.
    Success,
    /// The generic 'fatal error' message.
    ErrFatal,
    /// An invalid value was passed as an argument to a function.
    BadArgument,
    /// EEPROM status.
    Eeprom(Eeprom),
    /// There are no more buffers.
    NoBuffers,
    /// Serial status.
    Serial(Serial),
    /// MAC status.
    Mac(Mac),
    /// Simulated EEPROM status.
    SimEeprom(SimEeprom),
    /// Erroneous messages.
    Err(err::Err),
    /// The APS layer attempted to send or deliver a message, but it failed.
    DeliveryFailed,
    /// This binding index is out of range of the current binding table.
    BindingIndexOutOfRange,
    /// This address table index is out of range for the current address table.
    AddressTableIndexOutOfRange,
    /// An invalid binding table index was given to a function.
    InvalidBindingIndex,
    /// The API call is not allowed given the current state of the stack.
    InvalidCall,
    /// The link cost to a node is not known.
    CostNotKnown,
    /// The maximum number of in-flight messages (i.e. EMBER_APS_UNICAST_MESSAGE_COUNT) has been reached.
    MaxMessageLimitReached,
    /// The message to be transmitted is too big to fit into a single over-the-air packet.
    MessageTooLong,
    /// The application is trying to delete or overwrite a binding that is in use.
    BindingIsActive,
    /// The application is trying to overwrite an address table entry that is in use.
    AddressTableEntryIsActive,
    /// ADC status.
    Adc(Adc),
    /// Sleeping (for a duration) has been abnormally interrupted and exited prematurely.
    SleepInterrupted,
    /// PHY status.
    Phy(Phy),
    /// The stack software has completed initialization and is ready to send and receive packets over the air.
    NetworkUp,
    /// The network is not operating.
    NetworkDown,
    /// The node has not joined a network.
    NotJoined,
    /// An attempt to join a network failed.
    JoinFailed,
    /// The chosen security level (the value of `EMBER_SECURITY_LEVEL`) is not supported by the stack.
    InvalidSecurityLevel,
    /// After moving, a mobile node's attempt to re-establish contact with the network failed.
    MoveFailed,
    /// An attempt to join as a router failed due to a ZigBee versus ZigBee Pro incompatibility.
    ///
    /// ZigBee devices joining ZigBee Pro networks (or vice versa) must join as End Devices, not Routers.
    CannotJoinAsRouter,
    /// The local node ID has changed.
    ///
    /// The application can obtain the new node ID by calling `emberGetNodeId()`.
    NodeIdChanged,
    /// The local PAN ID has changed.
    ///
    /// The application can obtain the new PAN ID by calling `emberGetPanId()`.
    PanIdChanged,
    /// The network has been opened for joining.
    NetworkOpened,
    /// The network has been closed for joining.
    NetworkClosed,
    /// An attempt to join or rejoin the network failed because
    /// no router beacons could be heard by the joining node.
    NoBeacons,
    /// An attempt was made to join a Secured Network using a pre-configured key,
    /// but the Trust Center sent back a Network Key in-the-clear when an encrypted
    /// Network Key was required.
    ReceivedKeyInTheClear,
    /// An attempt was made to join a Secured Network, but the device did not receive a Network Key.
    NoNetworkKeyReceived,
    /// After a device joined a Secured Network, a Link Key was requested but no response was ever received.
    NoLinkKeyReceived,
    /// An attempt was made to join a Secured Network without a pre-configured key,
    /// but the Trust Center sent encrypted data using a pre-configured key.
    PreconfiguredKeyRequired,
    /// A message cannot be sent because the network is currently overloaded.
    NetworkBusy,
    /// The application tried to send a message using an endpoint that it has not defined.
    InvalidEndpoint,
    /// The application tried to use a binding that has been remotely modified
    /// and the change has not yet been reported to the application.
    BindingHasChanged,
    /// An attempt to generate random bytes failed because of insufficient random data from the radio.
    InsufficientRandomData,
    /// There was an error in trying to encrypt at the APS  Level.
    ///
    /// This could result from either an inability to determine the long address of the
    /// recipient from the short address (no entry in the binding table) or there is no
    /// link key entry in the table associated with the destination, or there was a failure
    /// to load the correct key into the encryption core.
    ApsEncryptionError,
    /// There was an attempt to form or join a network with security
    /// without calling `emberSetInitialSecurityState()` first.
    SecurityStateNotSet,
    /// A ZigBee route error command frame was received indicating that
    /// a source routed message from this node failed en route
    SourceRouteFailure,
    /// A ZigBee route error command frame was received indicating that a message sent
    /// to this node along a many-to-one route failed en route.
    ///
    /// The route error frame was delivered by an ad-hoc search for a functioning route.
    ManyToOneRouteFailure,
    /// A critical and fatal error indicating that the version of the stack trying
    /// to run does not match with the chip it is running on.
    ///
    /// The software (stack) on the chip must be replaced with software
    /// that is compatible with the chip.
    StackAndHardwareMismatch,
    /// An index was passed into the function that was larger than the valid range.
    IndexOutOfRange,
    /// There was an attempt to set an entry in the key table  using an invalid long address.
    ///
    /// An entry cannot be set using either the local device's or Trust Center's IEEE address.
    /// Or an entry already exists in the table with the same IEEE address.
    /// An Address of all zeros or  all F's are not valid addresses in 802.15.4.
    KeyTableInvalidAddress,
    /// There are no empty entries left in the table.
    TableFull,
    /// The requested function cannot be executed because the library that contains
    /// the necessary functionality is not present.
    LibraryNotPresent,
    /// The requested table entry has been erased and contains no valid data.
    TableEntryErased,
    /// There was an attempt to set a security configuration that is not valid
    /// given the other security settings.
    SecurityConfigurationInvalid,
    /// There was an attempt to broadcast a key switch too quickly after broadcasting the next network key.
    ///
    /// The Trust Center must wait at least a period equal to the broadcast timeout
    /// so that all routers have a chance to receive the broadcast of the new network key.
    TooSoonForSwitchKey,
    /// The stack accepted the command and is currently processing the request.
    ///
    /// The results will be returned via an appropriate handler.
    OperationInProgress,
    /// The message could not be sent because the link key corresponding to the destination
    /// is not authorized for use in APS data messages.
    ///
    /// APS Commands (sent by the stack) are allowed.
    /// To use it for encryption of APS data messages it must be authorized using
    /// a key agreement protocol (such as CBKE).
    KeyNotAuthorized,
    /// The security data provided was not valid, or an integrity check failed.
    SecurityDataInvalid,
    /// Application status.
    Application(Application),
}

impl Status {
    /// Checks the status for success and returns `Ok(value)` in that case.
    ///
    /// # Errors
    /// Returns `Err(self)` if the `Status` is not [`Status::Success`],
    pub fn map<T>(self, value: T) -> Result<T, Self> {
        if self == Self::Success {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// Checks the status for success and returns `Ok(())` in that case.
    ///
    /// # Errors
    /// Returns `Err(self)` if the `Status` is not [`Status::Success`],
    pub fn ok(self) -> Result<(), Self> {
        self.map(())
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "success"),
            Self::ErrFatal => write!(f, "fatal error"),
            Self::BadArgument => write!(f, "bad argument"),
            Self::Eeprom(eeprom) => write!(f, "EEPROM {eeprom}"),
            Self::NoBuffers => write!(f, "no buffers"),
            Self::Serial(serial) => write!(f, "serial {serial}"),
            Self::Mac(mac) => write!(f, "MAC {mac}"),
            Self::SimEeprom(sim_eeprom) => write!(f, "SIM/EEPROM {sim_eeprom}"),
            Self::Err(err) => write!(f, "error {err}"),
            Self::DeliveryFailed => write!(f, "delivery failed"),
            Self::BindingIndexOutOfRange => write!(f, "binding index out of range"),
            Self::AddressTableIndexOutOfRange => write!(f, "address table index out of range"),
            Self::InvalidBindingIndex => write!(f, "invalid binding index"),
            Self::InvalidCall => write!(f, "invalid call"),
            Self::CostNotKnown => write!(f, "cost not known"),
            Self::MaxMessageLimitReached => write!(f, "max message limit reached"),
            Self::MessageTooLong => write!(f, "message too long"),
            Self::BindingIsActive => write!(f, "binding is active"),
            Self::AddressTableEntryIsActive => write!(f, "address table entry is active"),
            Self::Adc(adc) => write!(f, "ADC {adc}"),
            Self::SleepInterrupted => write!(f, "sleep interrupted"),
            Self::Phy(phy) => write!(f, "phy {phy}"),
            Self::NetworkUp => write!(f, "network up"),
            Self::NetworkDown => write!(f, "network down"),
            Self::NotJoined => write!(f, "not joined"),
            Self::JoinFailed => write!(f, "join failed"),
            Self::InvalidSecurityLevel => write!(f, "invalid security level"),
            Self::MoveFailed => write!(f, "move failed"),
            Self::CannotJoinAsRouter => write!(f, "cannot join as router"),
            Self::NodeIdChanged => write!(f, "node id changed"),
            Self::PanIdChanged => write!(f, "PAN ID changed"),
            Self::NetworkOpened => write!(f, "network opened"),
            Self::NetworkClosed => write!(f, "network closed"),
            Self::NoBeacons => write!(f, "no beacons"),
            Self::ReceivedKeyInTheClear => write!(f, "received key in the clear"),
            Self::NoNetworkKeyReceived => write!(f, "no network key received"),
            Self::NoLinkKeyReceived => write!(f, "no link key received"),
            Self::PreconfiguredKeyRequired => write!(f, "preconfigured key required"),
            Self::NetworkBusy => write!(f, "network busy"),
            Self::InvalidEndpoint => write!(f, "invalid endpoint"),
            Self::BindingHasChanged => write!(f, "binding has changed"),
            Self::InsufficientRandomData => write!(f, "insufficient random data"),
            Self::ApsEncryptionError => write!(f, "APS encryption error"),
            Self::SecurityStateNotSet => write!(f, "security state not set"),
            Self::SourceRouteFailure => write!(f, "source route failure"),
            Self::ManyToOneRouteFailure => write!(f, "many to one route failure"),
            Self::StackAndHardwareMismatch => write!(f, "stack and hardware mismatch"),
            Self::IndexOutOfRange => write!(f, "index out of range"),
            Self::KeyTableInvalidAddress => write!(f, "key table invalid address"),
            Self::TableFull => write!(f, "table full"),
            Self::LibraryNotPresent => write!(f, "library not present"),
            Self::TableEntryErased => write!(f, "table entry erased"),
            Self::SecurityConfigurationInvalid => write!(f, "security configuration invalid"),
            Self::TooSoonForSwitchKey => write!(f, "too soon for switch key"),
            Self::OperationInProgress => write!(f, "operation in progress"),
            Self::KeyNotAuthorized => write!(f, "key not authorized"),
            Self::SecurityDataInvalid => write!(f, "security data invalid"),
            Self::Application(application) => write!(f, "application {application}"),
        }
    }
}

impl std::error::Error for Status {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Eeprom(eeprom) => Some(eeprom),
            Self::Serial(serial) => Some(serial),
            Self::Mac(mac) => Some(mac),
            Self::SimEeprom(sim_eeprom) => Some(sim_eeprom),
            Self::Err(err) => Some(err),
            Self::Adc(adc) => Some(adc),
            Self::Phy(phy) => Some(phy),
            Self::Application(application) => Some(application),
            _ => None,
        }
    }
}

impl From<Status> for Values {
    fn from(value: Status) -> Self {
        match value {
            Status::Success => Self::Success,
            Status::ErrFatal => Self::ErrFatal,
            Status::BadArgument => Self::BadArgument,
            Status::Eeprom(eeprom) => eeprom.into(),
            Status::NoBuffers => Self::NoBuffers,
            Status::Serial(serial) => serial.into(),
            Status::Mac(mac) => mac.into(),
            Status::SimEeprom(sim_eeprom) => sim_eeprom.into(),
            Status::Err(err) => err.into(),
            Status::DeliveryFailed => Self::DeliveryFailed,
            Status::BindingIndexOutOfRange => Self::BindingIndexOutOfRange,
            Status::AddressTableIndexOutOfRange => Self::AddressTableIndexOutOfRange,
            Status::InvalidBindingIndex => Self::InvalidBindingIndex,
            Status::InvalidCall => Self::InvalidCall,
            Status::CostNotKnown => Self::CostNotKnown,
            Status::MaxMessageLimitReached => Self::MaxMessageLimitReached,
            Status::MessageTooLong => Self::MessageTooLong,
            Status::BindingIsActive => Self::BindingIsActive,
            Status::AddressTableEntryIsActive => Self::AddressTableEntryIsActive,
            Status::Adc(adc) => adc.into(),
            Status::SleepInterrupted => Self::SleepInterrupted,
            Status::Phy(phy) => phy.into(),
            Status::NetworkUp => Self::NetworkUp,
            Status::NetworkDown => Self::NetworkDown,
            Status::NotJoined => Self::NotJoined,
            Status::JoinFailed => Self::JoinFailed,
            Status::InvalidSecurityLevel => Self::InvalidSecurityLevel,
            Status::MoveFailed => Self::MoveFailed,
            Status::CannotJoinAsRouter => Self::CannotJoinAsRouter,
            Status::NodeIdChanged => Self::NodeIdChanged,
            Status::PanIdChanged => Self::PanIdChanged,
            Status::NetworkOpened => Self::NetworkOpened,
            Status::NetworkClosed => Self::NetworkClosed,
            Status::NoBeacons => Self::NoBeacons,
            Status::ReceivedKeyInTheClear => Self::ReceivedKeyInTheClear,
            Status::NoNetworkKeyReceived => Self::NoNetworkKeyReceived,
            Status::NoLinkKeyReceived => Self::NoLinkKeyReceived,
            Status::PreconfiguredKeyRequired => Self::PreconfiguredKeyRequired,
            Status::NetworkBusy => Self::NetworkBusy,
            Status::InvalidEndpoint => Self::InvalidEndpoint,
            Status::BindingHasChanged => Self::BindingHasChanged,
            Status::InsufficientRandomData => Self::InsufficientRandomData,
            Status::ApsEncryptionError => Self::ApsEncryptionError,
            Status::SecurityStateNotSet => Self::SecurityStateNotSet,
            Status::SourceRouteFailure => Self::SourceRouteFailure,
            Status::ManyToOneRouteFailure => Self::ManyToOneRouteFailure,
            Status::StackAndHardwareMismatch => Self::StackAndHardwareMismatch,
            Status::IndexOutOfRange => Self::IndexOutOfRange,
            Status::KeyTableInvalidAddress => Self::KeyTableInvalidAddress,
            Status::TableFull => Self::TableFull,
            Status::LibraryNotPresent => Self::LibraryNotPresent,
            Status::TableEntryErased => Self::TableEntryErased,
            Status::SecurityConfigurationInvalid => Self::SecurityConfigurationInvalid,
            Status::TooSoonForSwitchKey => Self::TooSoonForSwitchKey,
            Status::OperationInProgress => Self::OperationInProgress,
            Status::KeyNotAuthorized => Self::KeyNotAuthorized,
            Status::SecurityDataInvalid => Self::SecurityDataInvalid,
            Status::Application(application) => application.into(),
        }
    }
}

impl From<Values> for Status {
    fn from(value: Values) -> Self {
        match value {
            Values::Success => Status::Success,
            Values::ErrFatal => Status::ErrFatal,
            Values::BadArgument => Status::BadArgument,
            Values::EepromMfgStackVersionMismatch => {
                Status::Eeprom(Eeprom::MfgStackVersionMismatch)
            }
            Values::EepromMfgVersionMismatch => Status::Eeprom(Eeprom::MfgVersionMismatch),
            Values::EepromStackVersionMismatch => Status::Eeprom(Eeprom::StackVersionMismatch),
            Values::NoBuffers => Status::NoBuffers,
            Values::SerialInvalidBaudRate => Status::Serial(Serial::InvalidBaudRate),
            Values::SerialInvalidPort => Status::Serial(Serial::InvalidPort),
            Values::SerialTxOverflow => Status::Serial(Serial::TxOverflow),
            Values::SerialRxOverflow => Status::Serial(Serial::RxOverflow),
            Values::SerialRxFrameError => Status::Serial(Serial::RxFrameError),
            Values::SerialRxParityError => Status::Serial(Serial::RxParityError),
            Values::SerialRxEmpty => Status::Serial(Serial::RxEmpty),
            Values::SerialRxOverrunError => Status::Serial(Serial::RxOverrunError),
            Values::MacNoData => Status::Mac(Mac::NoData),
            Values::MacJoinedNetwork => Status::Mac(Mac::JoinedNetwork),
            Values::MacBadScanDuration => Status::Mac(Mac::BadScanDuration),
            Values::MacIncorrectScanType => Status::Mac(Mac::IncorrectScanType),
            Values::MacInvalidChannelMask => Status::Mac(Mac::InvalidChannelMask),
            Values::MacCommandTransmitFailure => Status::Mac(Mac::CommandTransmitFailure),
            Values::MacTransmitQueueFull => Status::Mac(Mac::TransmitQueueFull),
            Values::MacUnknownHeaderType => Status::Mac(Mac::UnknownHeaderType),
            Values::MacScanning => Status::Mac(Mac::Scanning),
            Values::MacNoAckReceived => Status::Mac(Mac::NoAckReceived),
            Values::MacIndirectTimeout => Status::Mac(Mac::IndirectTimeout),
            Values::SimEepromErasePageGreen => Status::SimEeprom(SimEeprom::ErasePageGreen),
            Values::SimEepromErasePageRed => Status::SimEeprom(SimEeprom::ErasePageRed),
            Values::SimEepromFull => Status::SimEeprom(SimEeprom::Full),
            Values::SimEepromInit1Failed => Status::SimEeprom(SimEeprom::Init1Failed),
            Values::SimEepromInit2Failed => Status::SimEeprom(SimEeprom::Init2Failed),
            Values::SimEepromInit3Failed => Status::SimEeprom(SimEeprom::Init3Failed),
            Values::ErrFlashWriteInhibited => Status::Err(Err::Flash(Flash::WriteInhibited)),
            Values::ErrFlashVerifyFailed => Status::Err(Err::Flash(Flash::VerifyFailed)),
            Values::ErrFlashProgFail => Status::Err(Err::Flash(Flash::ProgFail)),
            Values::ErrFlashEraseFail => Status::Err(Err::Flash(Flash::EraseFail)),
            Values::ErrBootloaderTrapTableBad => {
                Status::Err(Err::Bootloader(Bootloader::TrapTableBad))
            }
            Values::ErrBootloaderTrapUnknown => {
                Status::Err(Err::Bootloader(Bootloader::TrapUnknown))
            }
            Values::ErrBootloaderNoImage => Status::Err(Err::Bootloader(Bootloader::NoImage)),
            Values::DeliveryFailed => Status::DeliveryFailed,
            Values::BindingIndexOutOfRange => Status::BindingIndexOutOfRange,
            Values::AddressTableIndexOutOfRange => Status::AddressTableIndexOutOfRange,
            Values::InvalidBindingIndex => Status::InvalidBindingIndex,
            Values::InvalidCall => Status::InvalidCall,
            Values::CostNotKnown => Status::CostNotKnown,
            Values::MaxMessageLimitReached => Status::MaxMessageLimitReached,
            Values::MessageTooLong => Status::MessageTooLong,
            Values::BindingIsActive => Status::BindingIsActive,
            Values::AddressTableEntryIsActive => Status::AddressTableEntryIsActive,
            Values::AdcConversionDone => Status::Adc(Adc::ConversionDone),
            Values::AdcConversionBusy => Status::Adc(Adc::ConversionBusy),
            Values::AdcConversionDeferred => Status::Adc(Adc::ConversionDeferred),
            Values::AdcNoConversionPending => Status::Adc(Adc::NoConversionPending),
            Values::SleepInterrupted => Status::SleepInterrupted,
            Values::PhyTxUnderflow => Status::Phy(Phy::TxUnderflow),
            Values::PhyTxIncomplete => Status::Phy(Phy::TxIncomplete),
            Values::PhyInvalidChannel => Status::Phy(Phy::InvalidChannel),
            Values::PhyInvalidPower => Status::Phy(Phy::InvalidPower),
            Values::PhyTxBusy => Status::Phy(Phy::TxBusy),
            Values::PhyTxCcaFail => Status::Phy(Phy::TxCcaFail),
            Values::PhyOscillatorCheckFailed => Status::Phy(Phy::OscillatorCheckFailed),
            Values::PhyAckReceived => Status::Phy(Phy::AckReceived),
            Values::NetworkUp => Status::NetworkUp,
            Values::NetworkDown => Status::NetworkDown,
            Values::NotJoined => Status::NotJoined,
            Values::JoinFailed => Status::JoinFailed,
            Values::InvalidSecurityLevel => Status::InvalidSecurityLevel,
            Values::MoveFailed => Status::MoveFailed,
            Values::CannotJoinAsRouter => Status::CannotJoinAsRouter,
            Values::NodeIdChanged => Status::NodeIdChanged,
            Values::PanIdChanged => Status::PanIdChanged,
            Values::NetworkOpened => Status::NetworkOpened,
            Values::NetworkClosed => Status::NetworkClosed,
            Values::NoBeacons => Status::NoBeacons,
            Values::ReceivedKeyInTheClear => Status::ReceivedKeyInTheClear,
            Values::NoNetworkKeyReceived => Status::NoNetworkKeyReceived,
            Values::NoLinkKeyReceived => Status::NoLinkKeyReceived,
            Values::PreconfiguredKeyRequired => Status::PreconfiguredKeyRequired,
            Values::NetworkBusy => Status::NetworkBusy,
            Values::InvalidEndpoint => Status::InvalidEndpoint,
            Values::BindingHasChanged => Status::BindingHasChanged,
            Values::InsufficientRandomData => Status::InsufficientRandomData,
            Values::ApsEncryptionError => Status::ApsEncryptionError,
            Values::SecurityStateNotSet => Status::SecurityStateNotSet,
            Values::SourceRouteFailure => Status::SourceRouteFailure,
            Values::ManyToOneRouteFailure => Status::ManyToOneRouteFailure,
            Values::StackAndHardwareMismatch => Status::StackAndHardwareMismatch,
            Values::IndexOutOfRange => Status::IndexOutOfRange,
            Values::KeyTableInvalidAddress => Status::KeyTableInvalidAddress,
            Values::TableFull => Status::TableFull,
            Values::LibraryNotPresent => Status::LibraryNotPresent,
            Values::TableEntryErased => Status::TableEntryErased,
            Values::SecurityConfigurationInvalid => Status::SecurityConfigurationInvalid,
            Values::TooSoonForSwitchKey => Status::TooSoonForSwitchKey,
            Values::OperationInProgress => Status::OperationInProgress,
            Values::KeyNotAuthorized => Status::KeyNotAuthorized,
            Values::SecurityDataInvalid => Status::SecurityDataInvalid,
            Values::ApplicationError0 => Status::Application(Application::Error0),
            Values::ApplicationError1 => Status::Application(Application::Error1),
            Values::ApplicationError2 => Status::Application(Application::Error2),
            Values::ApplicationError3 => Status::Application(Application::Error3),
            Values::ApplicationError4 => Status::Application(Application::Error4),
            Values::ApplicationError5 => Status::Application(Application::Error5),
            Values::ApplicationError6 => Status::Application(Application::Error6),
            Values::ApplicationError7 => Status::Application(Application::Error7),
            Values::ApplicationError8 => Status::Application(Application::Error8),
            Values::ApplicationError9 => Status::Application(Application::Error9),
            Values::ApplicationError10 => Status::Application(Application::Error10),
            Values::ApplicationError11 => Status::Application(Application::Error11),
            Values::ApplicationError12 => Status::Application(Application::Error12),
            Values::ApplicationError13 => Status::Application(Application::Error13),
            Values::ApplicationError14 => Status::Application(Application::Error14),
            Values::ApplicationError15 => Status::Application(Application::Error15),
        }
    }
}

impl From<Status> for u8 {
    fn from(status: Status) -> Self {
        match status {
            Status::Success => 0x00,
            Status::ErrFatal => 0x01,
            Status::BadArgument => 0x02,
            Status::Eeprom(eeprom) => eeprom.into(),
            Status::NoBuffers => 0x18,
            Status::Serial(serial) => serial.into(),
            Status::Mac(mac) => mac.into(),
            Status::SimEeprom(sim_eeprom) => sim_eeprom.into(),
            Status::Err(err) => err.into(),
            Status::DeliveryFailed => 0x66,
            Status::BindingIndexOutOfRange => 0x69,
            Status::AddressTableIndexOutOfRange => 0x6A,
            Status::InvalidBindingIndex => 0x6C,
            Status::InvalidCall => 0x70,
            Status::CostNotKnown => 0x71,
            Status::MaxMessageLimitReached => 0x72,
            Status::MessageTooLong => 0x74,
            Status::BindingIsActive => 0x75,
            Status::AddressTableEntryIsActive => 0x76,
            Status::Adc(adc) => adc.into(),
            Status::SleepInterrupted => 0x85,
            Status::Phy(phy) => phy.into(),
            Status::NetworkUp => 0x90,
            Status::NetworkDown => 0x91,
            Status::NotJoined => 0x93,
            Status::JoinFailed => 0x94,
            Status::InvalidSecurityLevel => 0x95,
            Status::MoveFailed => 0x96,
            Status::CannotJoinAsRouter => 0x98,
            Status::NodeIdChanged => 0x99,
            Status::PanIdChanged => 0x9A,
            Status::NetworkOpened => 0x9C,
            Status::NetworkClosed => 0x9D,
            Status::NoBeacons => 0xAB,
            Status::ReceivedKeyInTheClear => 0xAC,
            Status::NoNetworkKeyReceived => 0xAD,
            Status::NoLinkKeyReceived => 0xAE,
            Status::PreconfiguredKeyRequired => 0xAF,
            Status::NetworkBusy => 0xA1,
            Status::InvalidEndpoint => 0xA3,
            Status::BindingHasChanged => 0xA4,
            Status::InsufficientRandomData => 0xA5,
            Status::ApsEncryptionError => 0xA6,
            Status::SecurityStateNotSet => 0xA8,
            Status::SourceRouteFailure => 0xA9,
            Status::ManyToOneRouteFailure => 0xAA,
            Status::StackAndHardwareMismatch => 0xB0,
            Status::IndexOutOfRange => 0xB1,
            Status::KeyTableInvalidAddress => 0xB3,
            Status::TableFull => 0xB4,
            Status::LibraryNotPresent => 0xB5,
            Status::TableEntryErased => 0xB6,
            Status::SecurityConfigurationInvalid => 0xB7,
            Status::TooSoonForSwitchKey => 0xB8,
            Status::OperationInProgress => 0xBA,
            Status::KeyNotAuthorized => 0xBB,
            Status::SecurityDataInvalid => 0xBD,
            Status::Application(application) => application.into(),
        }
    }
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
            0x04..=0x07 => Eeprom::from_u8(n).map(Self::Eeprom),
            0x18 => Some(Self::NoBuffers),
            0x20..=0x27 => Serial::from_u8(n).map(Self::Serial),
            0x31..=0x42 => Mac::from_u8(n).map(Self::Mac),
            0x43..=0x45 | 0x48..=0x4A => SimEeprom::from_u8(n).map(Self::SimEeprom),
            0x46..=0x4C | 0x58..=0x5A => Err::from_u8(n).map(Self::Err),
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
            0x80..=0x84 => Adc::from_u8(n).map(Self::Adc),
            0x85 => Some(Self::SleepInterrupted),
            0x88..=0x8F => Phy::from_u8(n).map(Self::Phy),
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
            0xF0..=0xFF => Application::from_u8(n).map(Self::Application),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        u8::try_from(n).ok().and_then(Self::from_u8)
    }
}

impl Resolve for Result<Status, u8> {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        match self {
            Ok(status) => status.ok().map_err(Error::Ember),
            Err(status) => Err(Error::InvalidEmberStatus(status)),
        }
    }
}

impl TryFrom<u8> for Status {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
