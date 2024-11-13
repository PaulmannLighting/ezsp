//! Application Support Sublayer (APS) module.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// A list of APS options.
pub type Options = heapless::Vec<Option, 10>;

/// Ember APS option.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u16)]
pub enum Option {
    /// No options.
    None = 0x0000,
    /// Send the message using APS Encryption, using the Link Key shared with the
    /// destination node to encrypt the data at the APS Level.
    Encryption = 0x0020,
    /// Resend the message using the APS retry mechanism.
    Retry = 0x0040,
    /// Causes a route discovery to be initiated if no route to the destination is known.
    EnableRouteDiscovery = 0x0100,
    /// Causes a route discovery to be initiated even if one is known.
    ForceRouteDiscovery = 0x0200,
    /// Include the source EUI64 in the network frame.
    SourceEui64 = 0x0400,
    /// Include the destination EUI64 in the network frame.
    DestinationEui64 = 0x0800,
    /// Send a ZDO request to discover the node ID of the destination, if it is not already know.
    EnableAddressDiscovery = 0x1000,
    /// Reserved.
    PollResponse = 0x2000,
    /// This incoming message is a ZDO request not handled by the `EmberZNet` stack,
    /// and the application is responsible for sending a ZDO response.
    ///
    /// This flag is used only when the ZDO is configured to have requests handled by the application.
    /// See the `EZSP_CONFIG_APPLICATION_ZDO_FLAGS` configuration parameter for more information.
    ZdoResponseRequired = 0x4000,
    /// This message is part of a fragmented message. This option may only be set for unicasts.
    ///
    /// The `groupId` field gives the index of this fragment in the low-order byte.
    /// If the low-order byte is zero this is the first fragment and the high-order byte
    /// contains the number of fragments in the message.
    Fragment = 0x8000,
}

impl From<Option> for u16 {
    fn from(option: Option) -> Self {
        option as Self
    }
}

impl TryFrom<u16> for Option {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or(value)
    }
}

/// Zigbee APS frame parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Frame {
    profile_id: u16,
    cluster_id: u16,
    source_endpoint: u8,
    destination_endpoint: u8,
    options: u16,
    group_id: u16,
    sequence: u8,
}

impl Frame {
    /// Create a new APS frame.
    #[must_use]
    pub fn new(
        profile_id: u16,
        cluster_id: u16,
        source_endpoint: u8,
        destination_endpoint: u8,
        options: Options,
        group_id: u16,
        sequence: u8,
    ) -> Self {
        Self {
            profile_id,
            cluster_id,
            source_endpoint,
            destination_endpoint,
            options: options.into_iter().map(u16::from).sum(),
            group_id,
            sequence,
        }
    }

    /// Return the application profile ID that describes the format of the message.
    #[must_use]
    pub const fn profile_id(&self) -> u16 {
        self.profile_id
    }

    /// Return the cluster ID for this message.
    #[must_use]
    pub const fn cluster_id(&self) -> u16 {
        self.cluster_id
    }

    /// Return the source endpoint.
    #[must_use]
    pub const fn source_endpoint(&self) -> u8 {
        self.source_endpoint
    }

    /// Return the destination endpoint.
    #[must_use]
    pub const fn destination_endpoint(&self) -> u8 {
        self.destination_endpoint
    }

    /// Return a list of options.
    ///
    /// # Panics
    /// This function will panic if the options buffer does not have sufficient capacity.
    #[must_use]
    pub fn options(&self) -> Options {
        let mut options = Options::new();

        if Option::try_from(self.options) == Ok(Option::None) {
            return options;
        }

        if Option::Retry as u16 & self.options != 0 {
            options
                .push(Option::Retry)
                .expect("Options buffer should have sufficient capacity. This is a bug.");
        }

        if Option::EnableRouteDiscovery as u16 & self.options != 0 {
            options
                .push(Option::EnableRouteDiscovery)
                .expect("Options buffer should have sufficient capacity. This is a bug.");
        }

        if Option::ForceRouteDiscovery as u16 & self.options != 0 {
            options
                .push(Option::ForceRouteDiscovery)
                .expect("Options buffer should have sufficient capacity. This is a bug.");
        }

        if Option::SourceEui64 as u16 & self.options != 0 {
            options
                .push(Option::SourceEui64)
                .expect("Options buffer should have sufficient capacity. This is a bug.");
        }

        options
    }

    /// Return the group ID for this message, if it is multicast mode.
    #[must_use]
    pub const fn group_id(&self) -> u16 {
        self.group_id
    }

    /// Return the sequence number.
    #[must_use]
    pub const fn sequence(&self) -> u8 {
        self.sequence
    }
}
