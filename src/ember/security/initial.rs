//! Initial security state configuration for the EmberZNet stack.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::ember::key::Data;
use crate::ember::Eui64;

/// Ember initial security bitmask.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u16)]
pub enum Bitmask {
    /// This enables `ZigBee` Standard Security on the node.
    StandardSecurityMode = 0x0000,
    /// This enables Distributed Trust Center Mode for the device forming the network.
    /// (Previously known as `EMBER_NO_TRUST_CENTER_MODE`)
    DistributedTrustCenterMode = 0x0002,
    /// This enables a Global Link Key for the Trust Center.
    /// All nodes will share the same Trust Center Link Key.
    TrustCenterGlobalLinkKey = 0x0004,
    /// This enables devices that perform MAC Association with a pre-configured
    /// Network Key to join the network.
    /// It is only set on the Trust Center.
    PreconfiguredNetworkKeyMode = 0x0008,
    /// This denotes that the preconfiguredKey is not the actual Link Key
    /// but a Secret Key known only to the Trust Center.
    ///
    /// It is hashed with the IEEE Address of the destination device in order to
    /// create the actual Link Key used in encryption.
    /// This bit is only used by the Trust Center.
    /// The joining device need not set this.
    TrustCenterUsesHashedLinkKey = 0x0084,
    /// This denotes that the preconfiguredKey element has valid data that should
    /// be used to configure the initial security state.
    HavePreconfiguredKey = 0x0100,
    /// This denotes that the networkKey element has valid data that should
    /// be used to configure the initial security state.
    HaveNetworkKey = 0x0200,
    /// This denotes to a joining node that it should attempt to acquire a Trust Center Link Key
    /// during joining.
    ///
    /// This is only necessary if the device does not have a pre-configured key.
    GetLinkKeyWhenJoining = 0x0400,
    /// This denotes that a joining device should only accept an encrypted network key
    /// from the Trust Center (using its pre-configured key).
    ///
    /// A key sent in-the-clear by the Trust Center will be rejected and the join will fail.
    /// This option is only valid when utilizing a pre-configured key.
    RequireEncryptedKey = 0x0800,
    /// This denotes whether the device should NOT reset its  outgoing frame counters
    /// (both NWK and APS) when `emberSetInitialSecurityState()` is called.
    ///
    /// Normally it is advised to reset the frame counter before joining a new network.
    /// However, in cases where a device is joining to the same network again
    /// (but not using `emberRejoinNetwork()`) it should keep the NWK and APS frame counters
    /// stored in its tokens.
    NoFrameCounterReset = 0x1000,
    /// This denotes that the device should obtain its preconfigured key from an installation code
    /// stored in the manufacturing token.
    ///
    /// The token contains a value that will be hashed to obtain the actual preconfigured key.
    /// If that token is not valid, then the call to `emberSetInitialSecurityState()` will fail.
    GetPreconfiguredKeyFromInstallCode = 0x2000,
    /// This denotes that the `EmberInitialSecurityState::preconfiguredTrustCenterEui64`
    /// has a value in it containing the trust center EUI64.
    ///
    /// The device will only join a network and accept commands from a trust center with that EUI64.
    /// Normally this bit is NOT set, and the EUI64 of the trust center is learned during the join process.
    /// When commissioning a device to join onto an existing network, which is using a trust center,
    /// and without sending any messages, this bit must be set and the field
    /// `EmberInitialSecurityState::preconfiguredTrustCenterEui64` must be populated with the
    /// appropriate EUI64.
    HaveTrustCenterEui64 = 0x0040,
}

impl From<Bitmask> for u16 {
    fn from(bitmask: Bitmask) -> Self {
        bitmask as Self
    }
}

impl TryFrom<u16> for Bitmask {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or(value)
    }
}

/// The security data used to set the configuration for the stack,
/// or the retrieved configuration currently in use.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct State {
    bitmask: u16,
    preconfigured_key: Data,
    network_key: Data,
    network_key_sequence_number: u8,
    preconfigured_trust_center_eui64: Eui64,
}

impl State {
    /// Create a new security state.
    #[must_use]
    pub fn new(
        bitmask: Bitmask,
        preconfigured_key: Data,
        network_key: Data,
        network_key_sequence_number: u8,
        preconfigured_trust_center_eui64: Eui64,
    ) -> Self {
        Self {
            bitmask: bitmask.into(),
            preconfigured_key,
            network_key,
            network_key_sequence_number,
            preconfigured_trust_center_eui64,
        }
    }

    /// Return the bitmask indicating the security state used to indicate what the
    /// security configuration will be when the device forms or joins the network.
    ///
    /// # Errors
    /// Returns the [`u8`] value of the bitmask if it is not a valid [`Bitmask`].
    pub fn bitmask(&self) -> Result<Bitmask, u16> {
        Bitmask::try_from(self.bitmask)
    }

    /// Return the pre-configured Key data that should be used when forming or joining the network.
    ///
    /// The security bitmask must be set with the `EMBER_HAVE_PRECONFIGURED_KEY` bit
    /// to indicate that the key contains valid data.
    #[must_use]
    pub const fn preconfigured_key(&self) -> &Data {
        &self.preconfigured_key
    }

    /// Return the Network Key that should be used by the Trust Center when it forms the network,
    /// or the Network Key currently in use by a joined device.
    ///
    /// The security bitmask must be set with `EMBER_HAVE_NETWORK_KEY`
    /// to indicate that the key contains valid data.
    #[must_use]
    pub const fn network_key(&self) -> &Data {
        &self.network_key
    }

    /// Return the sequence number associated with the network key.
    ///
    /// This is only valid if the `EMBER_HAVE_NETWORK_KEY` has been set in the security bitmask.
    #[must_use]
    pub const fn network_key_sequence_number(&self) -> u8 {
        self.network_key_sequence_number
    }

    /// Return this is the long address of the trust center on the network that will be joined.
    ///
    /// It is usually NOT set prior to joining the network and instead it is learned during
    /// the joining message exchange.
    /// This field is only examined if `EMBER_HAVE_TRUST_CENTER_EUI64` is set in  the
    /// `EmberInitialSecurityState::bitmask`.
    /// Most devices should clear that bit and leave this field alone.
    /// This field must be set when using commissioning mode.
    #[must_use]
    pub const fn preconfigured_trust_center_eui64(&self) -> Eui64 {
        self.preconfigured_trust_center_eui64
    }
}
