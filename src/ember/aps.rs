//! Application Support Sublayer (APS) module.

use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

/// Ember APS options.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct Options(u16);
bitflags! {
    impl Options: u16 {
        /// No options.
        const NONE = 0x0000;
        /// Send the message using APS Encryption, using the Link Key shared with the
        /// destination node to encrypt the data at the APS Level.
        const ENCRYPTION = 0x0020;
        /// Resend the message using the APS retry mechanism.
        const RETRY = 0x0040;
        /// Causes a route discovery to be initiated if no route to the destination is known.
        const ENABLE_ROUTE_DISCOVERY = 0x0100;
        /// Causes a route discovery to be initiated even if one is known.
        const FORCE_ROUTE_DISCOVERY = 0x0200;
        /// Include the source EUI64 in the network frame.
        const SOURCE_EUI64 = 0x0400;
        /// Include the destination EUI64 in the network frame.
        const DESTINATION_EUI64 = 0x0800;
        /// Send a ZDO request to discover the node ID of the destination, if it is not already know.
        const ENABLE_ADDRESS_DISCOVERY = 0x1000;
        /// Reserved.
        const POLL_RESPONSE = 0x2000;
        /// This incoming message is a ZDO request not handled by the `EmberZNet` stack,
        /// and the application is responsible for sending a ZDO response.
        ///
        /// This flag is used only when the ZDO is configured to have requests handled by the application.
        /// See the [`ApplicationZdoFlags`](crate::ezsp::config::Id::ApplicationZdoFlags) configuration parameter for more information.
        const ZDO_RESPONSE_REQUIRED = 0x4000;
        /// This message is part of a fragmented message. This option may only be set for unicasts.
        ///
        /// The `groupId` field gives the index of this fragment in the low-order byte.
        /// If the low-order byte is zero this is the first fragment and the high-order byte
        /// contains the number of fragments in the message.
         const FRAGMENT = 0x8000;
    }
}

impl From<Options> for u16 {
    fn from(options: Options) -> Self {
        options.0
    }
}

impl From<u16> for Options {
    fn from(value: u16) -> Self {
        Self::from_bits_truncate(value)
    }
}

/// Zigbee APS frame parameters.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct Frame {
    profile_id: u16,
    cluster_id: u16,
    source_endpoint: u8,
    destination_endpoint: u8,
    options: Options,
    group_id: u16,
    sequence: u8,
}

impl Frame {
    /// Create a new APS frame.
    #[must_use]
    pub const fn new(
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
            options,
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
    #[must_use]
    pub const fn options(&self) -> Options {
        self.options
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

    /// Return fragmentation information, if the message is fragmented.
    ///
    /// # Returns
    ///
    /// - `Some((index, Some(total_fragments)))` if this is the first fragment, where `index` is 0.
    /// - `Some((index, None))` if this is a subsequent fragment.
    /// - `None` if the message is not fragmented.
    #[must_use]
    pub const fn fragmentation(&self) -> Option<(u8, Option<u8>)> {
        if self.options.contains(Options::FRAGMENT) {
            let index = (self.group_id & 0x00FF) as u8;

            if index == 0 {
                let total_fragments = (self.group_id >> 8) as u8;
                Some((index, Some(total_fragments)))
            } else {
                Some((index, None))
            }
        } else {
            None
        }
    }
}
