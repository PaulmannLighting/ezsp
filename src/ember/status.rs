use core::fmt::{Debug, Display, Formatter, LowerHex, UpperHex};

pub use adc::Adc;
pub use application::Application;
pub use eeprom::Eeprom;
pub use err::{Bootloader, Err, Flash};
pub use mac::Mac;
use num_traits::FromPrimitive;
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
///
/// # Documentation
///
/// See <https://www.silabs.com/documents/public/miscellaneous/EmberZNet-API-EM260.pdf> for details.
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
    Err(Err),
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
    // TODO: Where is `EMBER_APS_UNICAST_MESSAGE_COUNT` defined?
    /// The maximum number of in-flight messages (i.e. `EMBER_APS_UNICAST_MESSAGE_COUNT`) has been reached.
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
    /// An attempt to join as a router failed due to a Zigbee versus Zigbee Pro incompatibility.
    ///
    /// Zigbee devices joining Zigbee Pro networks (or vice versa) must join as End Devices, not Routers.
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
    /// The passed key data is not valid.
    ///
    /// A key of all zeros or all F's are reserved values and cannot be used.
    KeyInvalid,
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
    /// A Zigbee route error command frame was received indicating that
    /// a source routed message from this node failed en route
    SourceRouteFailure,
    /// A Zigbee route error command frame was received indicating that a message sent
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
    /// The received signature corresponding to the message that was passed
    /// to the `CBKE` Library failed verification, it is not valid.
    SignatureVerifyFailure,
    /// The stack accepted the command and is currently processing the request.
    ///
    /// The results will be returned via an appropriate handler.
    OperationInProgress,
    /// The message could not be sent because the link key corresponding to the destination
    /// is not authorized for use in APS data messages.
    ///
    /// APS Commands (sent by the stack) are allowed.
    /// To use it for encryption of APS data messages it must be authorized using
    /// a key agreement protocol (such as `CBKE`).
    KeyNotAuthorized,
    /// The security data provided was not valid, or an integrity check failed.
    SecurityDataInvalid,
    /// Application status.
    Application(Application),
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&Values::from(*self), f)
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
            Status::KeyInvalid => Self::KeyInvalid,
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
            Status::SignatureVerifyFailure => Self::SignatureVerifyFailure,
            Status::OperationInProgress => Self::OperationInProgress,
            Status::KeyNotAuthorized => Self::KeyNotAuthorized,
            Status::SecurityDataInvalid => Self::SecurityDataInvalid,
            Status::Application(application) => application.into(),
        }
    }
}

impl From<Values> for Status {
    #[allow(clippy::too_many_lines)]
    fn from(value: Values) -> Self {
        match value {
            Values::Success => Self::Success,
            Values::ErrFatal => Self::ErrFatal,
            Values::BadArgument => Self::BadArgument,
            Values::EepromMfgStackVersionMismatch => Self::Eeprom(Eeprom::MfgStackVersionMismatch),
            Values::EepromMfgVersionMismatch => Self::Eeprom(Eeprom::MfgVersionMismatch),
            Values::EepromStackVersionMismatch => Self::Eeprom(Eeprom::StackVersionMismatch),
            Values::NoBuffers => Self::NoBuffers,
            Values::SerialInvalidBaudRate => Self::Serial(Serial::InvalidBaudRate),
            Values::SerialInvalidPort => Self::Serial(Serial::InvalidPort),
            Values::SerialTxOverflow => Self::Serial(Serial::TxOverflow),
            Values::SerialRxOverflow => Self::Serial(Serial::RxOverflow),
            Values::SerialRxFrameError => Self::Serial(Serial::RxFrameError),
            Values::SerialRxParityError => Self::Serial(Serial::RxParityError),
            Values::SerialRxEmpty => Self::Serial(Serial::RxEmpty),
            Values::SerialRxOverrunError => Self::Serial(Serial::RxOverrunError),
            Values::MacNoData => Self::Mac(Mac::NoData),
            Values::MacJoinedNetwork => Self::Mac(Mac::JoinedNetwork),
            Values::MacBadScanDuration => Self::Mac(Mac::BadScanDuration),
            Values::MacIncorrectScanType => Self::Mac(Mac::IncorrectScanType),
            Values::MacInvalidChannelMask => Self::Mac(Mac::InvalidChannelMask),
            Values::MacCommandTransmitFailure => Self::Mac(Mac::CommandTransmitFailure),
            Values::MacTransmitQueueFull => Self::Mac(Mac::TransmitQueueFull),
            Values::MacUnknownHeaderType => Self::Mac(Mac::UnknownHeaderType),
            Values::MacScanning => Self::Mac(Mac::Scanning),
            Values::MacNoAckReceived => Self::Mac(Mac::NoAckReceived),
            Values::MacIndirectTimeout => Self::Mac(Mac::IndirectTimeout),
            Values::SimEepromErasePageGreen => Self::SimEeprom(SimEeprom::ErasePageGreen),
            Values::SimEepromErasePageRed => Self::SimEeprom(SimEeprom::ErasePageRed),
            Values::SimEepromFull => Self::SimEeprom(SimEeprom::Full),
            Values::SimEepromInit1Failed => Self::SimEeprom(SimEeprom::Init1Failed),
            Values::SimEepromInit2Failed => Self::SimEeprom(SimEeprom::Init2Failed),
            Values::SimEepromInit3Failed => Self::SimEeprom(SimEeprom::Init3Failed),
            Values::ErrFlashWriteInhibited => Self::Err(Err::Flash(Flash::WriteInhibited)),
            Values::ErrFlashVerifyFailed => Self::Err(Err::Flash(Flash::VerifyFailed)),
            Values::ErrFlashProgFail => Self::Err(Err::Flash(Flash::ProgFail)),
            Values::ErrFlashEraseFail => Self::Err(Err::Flash(Flash::EraseFail)),
            Values::ErrBootloaderTrapTableBad => {
                Self::Err(Err::Bootloader(Bootloader::TrapTableBad))
            }
            Values::ErrBootloaderTrapUnknown => Self::Err(Err::Bootloader(Bootloader::TrapUnknown)),
            Values::ErrBootloaderNoImage => Self::Err(Err::Bootloader(Bootloader::NoImage)),
            Values::DeliveryFailed => Self::DeliveryFailed,
            Values::BindingIndexOutOfRange => Self::BindingIndexOutOfRange,
            Values::AddressTableIndexOutOfRange => Self::AddressTableIndexOutOfRange,
            Values::InvalidBindingIndex => Self::InvalidBindingIndex,
            Values::InvalidCall => Self::InvalidCall,
            Values::CostNotKnown => Self::CostNotKnown,
            Values::MaxMessageLimitReached => Self::MaxMessageLimitReached,
            Values::MessageTooLong => Self::MessageTooLong,
            Values::BindingIsActive => Self::BindingIsActive,
            Values::AddressTableEntryIsActive => Self::AddressTableEntryIsActive,
            Values::AdcConversionDone => Self::Adc(Adc::ConversionDone),
            Values::AdcConversionBusy => Self::Adc(Adc::ConversionBusy),
            Values::AdcConversionDeferred => Self::Adc(Adc::ConversionDeferred),
            Values::AdcNoConversionPending => Self::Adc(Adc::NoConversionPending),
            Values::SleepInterrupted => Self::SleepInterrupted,
            Values::PhyTxUnderflow => Self::Phy(Phy::TxUnderflow),
            Values::PhyTxIncomplete => Self::Phy(Phy::TxIncomplete),
            Values::PhyInvalidChannel => Self::Phy(Phy::InvalidChannel),
            Values::PhyInvalidPower => Self::Phy(Phy::InvalidPower),
            Values::PhyTxBusy => Self::Phy(Phy::TxBusy),
            Values::PhyTxCcaFail => Self::Phy(Phy::TxCcaFail),
            Values::PhyOscillatorCheckFailed => Self::Phy(Phy::OscillatorCheckFailed),
            Values::PhyAckReceived => Self::Phy(Phy::AckReceived),
            Values::NetworkUp => Self::NetworkUp,
            Values::NetworkDown => Self::NetworkDown,
            Values::NotJoined => Self::NotJoined,
            Values::JoinFailed => Self::JoinFailed,
            Values::InvalidSecurityLevel => Self::InvalidSecurityLevel,
            Values::MoveFailed => Self::MoveFailed,
            Values::CannotJoinAsRouter => Self::CannotJoinAsRouter,
            Values::NodeIdChanged => Self::NodeIdChanged,
            Values::PanIdChanged => Self::PanIdChanged,
            Values::NetworkOpened => Self::NetworkOpened,
            Values::NetworkClosed => Self::NetworkClosed,
            Values::NoBeacons => Self::NoBeacons,
            Values::ReceivedKeyInTheClear => Self::ReceivedKeyInTheClear,
            Values::NoNetworkKeyReceived => Self::NoNetworkKeyReceived,
            Values::NoLinkKeyReceived => Self::NoLinkKeyReceived,
            Values::PreconfiguredKeyRequired => Self::PreconfiguredKeyRequired,
            Values::KeyInvalid => Self::KeyInvalid,
            Values::NetworkBusy => Self::NetworkBusy,
            Values::InvalidEndpoint => Self::InvalidEndpoint,
            Values::BindingHasChanged => Self::BindingHasChanged,
            Values::InsufficientRandomData => Self::InsufficientRandomData,
            Values::ApsEncryptionError => Self::ApsEncryptionError,
            Values::SecurityStateNotSet => Self::SecurityStateNotSet,
            Values::SourceRouteFailure => Self::SourceRouteFailure,
            Values::ManyToOneRouteFailure => Self::ManyToOneRouteFailure,
            Values::StackAndHardwareMismatch => Self::StackAndHardwareMismatch,
            Values::IndexOutOfRange => Self::IndexOutOfRange,
            Values::KeyTableInvalidAddress => Self::KeyTableInvalidAddress,
            Values::TableFull => Self::TableFull,
            Values::LibraryNotPresent => Self::LibraryNotPresent,
            Values::TableEntryErased => Self::TableEntryErased,
            Values::SecurityConfigurationInvalid => Self::SecurityConfigurationInvalid,
            Values::TooSoonForSwitchKey => Self::TooSoonForSwitchKey,
            Values::SignatureVerifyFailure => Self::SignatureVerifyFailure,
            Values::OperationInProgress => Self::OperationInProgress,
            Values::KeyNotAuthorized => Self::KeyNotAuthorized,
            Values::SecurityDataInvalid => Self::SecurityDataInvalid,
            Values::ApplicationError0 => Self::Application(Application::Error0),
            Values::ApplicationError1 => Self::Application(Application::Error1),
            Values::ApplicationError2 => Self::Application(Application::Error2),
            Values::ApplicationError3 => Self::Application(Application::Error3),
            Values::ApplicationError4 => Self::Application(Application::Error4),
            Values::ApplicationError5 => Self::Application(Application::Error5),
            Values::ApplicationError6 => Self::Application(Application::Error6),
            Values::ApplicationError7 => Self::Application(Application::Error7),
            Values::ApplicationError8 => Self::Application(Application::Error8),
            Values::ApplicationError9 => Self::Application(Application::Error9),
            Values::ApplicationError10 => Self::Application(Application::Error10),
            Values::ApplicationError11 => Self::Application(Application::Error11),
            Values::ApplicationError12 => Self::Application(Application::Error12),
            Values::ApplicationError13 => Self::Application(Application::Error13),
            Values::ApplicationError14 => Self::Application(Application::Error14),
            Values::ApplicationError15 => Self::Application(Application::Error15),
        }
    }
}

impl From<Status> for u8 {
    fn from(status: Status) -> Self {
        Values::from(status).into()
    }
}

impl FromPrimitive for Status {
    fn from_i64(n: i64) -> Option<Self> {
        Values::from_i64(n).map(Self::from)
    }

    fn from_u8(n: u8) -> Option<Self> {
        Values::from_u8(n).map(Self::from)
    }

    fn from_u64(n: u64) -> Option<Self> {
        Values::from_u64(n).map(Self::from)
    }
}

impl LowerHex for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        LowerHex::fmt(&Values::from(*self), f)
    }
}

impl UpperHex for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        UpperHex::fmt(&Values::from(*self), f)
    }
}
