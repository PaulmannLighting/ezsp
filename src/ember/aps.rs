use le_stream::derive::{FromLeBytes, ToLeBytes};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub type Options = heapless::Vec<Option, 10>;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u16)]
pub enum Option {
    None = 0x0000,
    Encryption = 0x0020,
    Retry = 0x0040,
    EnableRouteDiscovery = 0x0100,
    ForceRouteDiscovery = 0x0200,
    SourceEui64 = 0x0400,
    DestinationEui64 = 0x0800,
    EnableAddressDiscovery = 0x1000,
    PollResponse = 0x2000,
    ZdoResponseRequired = 0x4000,
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

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
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

    #[must_use]
    pub const fn profile_id(&self) -> u16 {
        self.profile_id
    }

    #[must_use]
    pub const fn cluster_id(&self) -> u16 {
        self.cluster_id
    }

    #[must_use]
    pub const fn source_endpoint(&self) -> u8 {
        self.source_endpoint
    }

    #[must_use]
    pub const fn destination_endpoint(&self) -> u8 {
        self.destination_endpoint
    }

    /// Returns the options set in the frame.
    ///
    /// # Panics
    /// This function will panic if the options buffer does not have sufficient capacity.
    #[must_use]
    pub fn options(&self) -> Options {
        let mut options = Options::new();

        if Option::try_from(self.options) == Ok(Option::None) {
            return options;
        }

        if Into::<u16>::into(Option::Retry) & self.options != 0 {
            options
                .push(Option::Retry)
                .expect("Options buffer should have sufficient capacity.");
        }

        if Into::<u16>::into(Option::EnableRouteDiscovery) & self.options != 0 {
            options
                .push(Option::EnableRouteDiscovery)
                .expect("Options buffer should have sufficient capacity.");
        }

        if Into::<u16>::into(Option::ForceRouteDiscovery) & self.options != 0 {
            options
                .push(Option::ForceRouteDiscovery)
                .expect("Options buffer should have sufficient capacity.");
        }

        if Into::<u16>::into(Option::SourceEui64) & self.options != 0 {
            options
                .push(Option::SourceEui64)
                .expect("Options buffer should have sufficient capacity.");
        }

        options
    }

    #[must_use]
    pub const fn group_id(&self) -> u16 {
        self.group_id
    }

    #[must_use]
    pub const fn sequence(&self) -> u8 {
        self.sequence
    }
}
