use std::fmt::{Debug, Display, Formatter};

use num_traits::FromPrimitive;

pub use adc::Adc;
pub use application::Application;
pub use eeprom::Eeprom;
pub use err::{Bootloader, Err, Flash};
pub use mac::Mac;
pub use phy::Phy;
pub use serial::Serial;
pub use sim_eeprom::SimEeprom;

use crate::Error;
use crate::Resolve;

mod adc;
mod application;
mod eeprom;
mod err;
mod mac;
mod phy;
mod serial;
mod sim_eeprom;

/// Ember status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Status {
    /// The generic 'no error' message.
    Success = 0x00,
    /// The generic 'fatal error' message.
    ErrFatal = 0x01,
    /// An invalid value was passed as an argument to a function.
    BadArgument = 0x02,
    /// EEPROM status.
    Eeprom(Eeprom),
    /// There are no more buffers.
    NoBuffers = 0x18,
    /// Serial status.
    Serial(Serial),
    /// MAC status.
    Mac(Mac),
    /// Simulated EEPROM status.
    SimEeprom(SimEeprom),
    /// Erroneous messages.
    Err(err::Err),
    /// The APS layer attempted to send or deliver a message, but it failed.
    DeliveryFailed = 0x66,
    /// This binding index is out of range of the current binding table.
    BindingIndexOutOfRange = 0x69,
    /// This address table index is out of range for the current address table.
    AddressTableIndexOutOfRange = 0x6A,
    /// An invalid binding table index was given to a function.
    InvalidBindingIndex = 0x6C,
    /// The API call is not allowed given the current state of the stack.
    InvalidCall = 0x70,
    /// The link cost to a node is not known.
    CostNotKnown = 0x71,
    /// The maximum number of in-flight messages (i.e. EMBER_APS_UNICAST_MESSAGE_COUNT) has been reached.
    MaxMessageLimitReached = 0x72,
    /// The message to be transmitted is too big to fit into a single over-the-air packet.
    MessageTooLong = 0x74,
    /// The application is trying to delete or overwrite a binding that is in use.
    BindingIsActive = 0x75,
    /// The application is trying to overwrite an address table entry that is in use.
    AddressTableEntryIsActive = 0x76,
    /// ADC status.
    Adc(Adc),
    /// Sleeping (for a duration) has been abnormally interrupted and exited prematurely.
    SleepInterrupted = 0x85,
    /// PHY status.
    Phy(Phy),
    /// The stack software has completed initialization and is ready to send and receive packets over the air.
    NetworkUp = 0x90,
    /// The network is not operating.
    NetworkDown = 0x91,
    /// The node has not joined a network.
    NotJoined = 0x93,
    /// An attempt to join a network failed.
    JoinFailed = 0x94,
    /// The chosen security level (the value of `EMBER_SECURITY_LEVEL`) is not supported by the stack.
    InvalidSecurityLevel = 0x95,
    /// After moving, a mobile node's attempt to re-establish contact with the network failed.
    MoveFailed = 0x96,
    /// An attempt to join as a router failed due to a ZigBee versus ZigBee Pro incompatibility.
    ///
    /// ZigBee devices joining ZigBee Pro networks (or vice versa) must join as End Devices, not Routers.
    CannotJoinAsRouter = 0x98,
    /// The local node ID has changed.
    ///
    /// The application can obtain the new node ID by calling `emberGetNodeId()`.
    NodeIdChanged = 0x99,
    /// The local PAN ID has changed.
    ///
    /// The application can obtain the new PAN ID by calling `emberGetPanId()`.
    PanIdChanged = 0x9A,
    /// The network has been opened for joining.
    NetworkOpened = 0x9C,
    /// The network has been closed for joining.
    NetworkClosed = 0x9D,
    /// An attempt to join or rejoin the network failed because
    /// no router beacons could be heard by the joining node.
    NoBeacons = 0xAB,
    /// An attempt was made to join a Secured Network using a pre-configured key,
    /// but the Trust Center sent back a Network Key in-the-clear when an encrypted
    /// Network Key was required.
    ReceivedKeyInTheClear = 0xAC,
    /// An attempt was made to join a Secured Network, but the device did not receive a Network Key.
    NoNetworkKeyReceived = 0xAD,
    /// After a device joined a Secured Network, a Link Key was requested but no response was ever received.
    NoLinkKeyReceived = 0xAE,
    /// An attempt was made to join a Secured Network without a pre-configured key,
    /// but the Trust Center sent encrypted data using a pre-configured key.
    PreconfiguredKeyRequired = 0xAF,
    /// A message cannot be sent because the network is currently overloaded.
    NetworkBusy = 0xA1,
    /// The application tried to send a message using an endpoint that it has not defined.
    InvalidEndpoint = 0xA3,
    /// The application tried to use a binding that has been remotely modified
    /// and the change has not yet been reported to the application.
    BindingHasChanged = 0xA4,
    /// An attempt to generate random bytes failed because of insufficient random data from the radio.
    InsufficientRandomData = 0xA5,
    /// There was an error in trying to encrypt at the APS  Level.
    ///
    /// This could result from either an inability to determine the long address of the
    /// recipient from the short address (no entry in the binding table) or there is no
    /// link key entry in the table associated with the destination, or there was a failure
    /// to load the correct key into the encryption core.
    ApsEncryptionError = 0xA6,
    /// There was an attempt to form or join a network with security
    /// without calling `emberSetInitialSecurityState()` first.
    SecurityStateNotSet = 0xA8,
    /// A ZigBee route error command frame was received indicating that
    /// a source routed message from this node failed en route
    SourceRouteFailure = 0xA9,
    /// A ZigBee route error command frame was received indicating that a message sent
    /// to this node along a many-to-one route failed en route.
    ///
    /// The route error frame was delivered by an ad-hoc search for a functioning route.
    ManyToOneRouteFailure = 0xAA,
    /// A critical and fatal error indicating that the version of the stack trying
    /// to run does not match with the chip it is running on.
    ///
    /// The software (stack) on the chip must be replaced with software
    /// that is compatible with the chip.
    StackAndHardwareMismatch = 0xB0,
    /// An index was passed into the function that was larger than the valid range.
    IndexOutOfRange = 0xB1,
    /// There was an attempt to set an entry in the key table  using an invalid long address.
    ///
    /// An entry cannot be set using either the local device's or Trust Center's IEEE address.
    /// Or an entry already exists in the table with the same IEEE address.
    /// An Address of all zeros or  all F's are not valid addresses in 802.15.4.
    KeyTableInvalidAddress = 0xB3,
    /// There are no empty entries left in the table.
    TableFull = 0xB4,
    /// The requested function cannot be executed because the library that contains
    /// the necessary functionality is not present.
    LibraryNotPresent = 0xB5,
    /// The requested table entry has been erased and contains no valid data.
    TableEntryErased = 0xB6,
    /// There was an attempt to set a security configuration that is not valid
    /// given the other security settings.
    SecurityConfigurationInvalid = 0xB7,
    /// There was an attempt to broadcast a key switch too quickly after broadcasting the next network key.
    ///
    /// The Trust Center must wait at least a period equal to the broadcast timeout
    /// so that all routers have a chance to receive the broadcast of the new network key.
    TooSoonForSwitchKey = 0xB8,
    /// The stack accepted the command and is currently processing the request.
    ///
    /// The results will be returned via an appropriate handler.
    OperationInProgress = 0xBA,
    /// The message could not be sent because the link key corresponding to the destination
    /// is not authorized for use in APS data messages.
    ///
    /// APS Commands (sent by the stack) are allowed.
    /// To use it for encryption of APS data messages it must be authorized using
    /// a key agreement protocol (such as CBKE).
    KeyNotAuthorized = 0xBB,
    /// The security data provided was not valid, or an integrity check failed.
    SecurityDataInvalid = 0xBD,
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
