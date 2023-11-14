use le_stream::derive::{FromLeBytes, ToLeBytes};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

pub type Options = heapless::Vec<Option, 5>;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Option {
    None = 0x0000,
    Encryption = 0x0020,
    Retry = 0x0040,
    EnableRouteDiscovery = 0x0100,
    ForceRouteDiscovery = 0x0200,
    SourceEui64 = 0x0400,
}

impl From<Option> for u16 {
    fn from(option: Option) -> Self {
        option.to_u16().expect("could not convert Option to u8")
    }
}

impl TryFrom<u16> for Option {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or(value)
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
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
        options: Option,
        group_id: u16,
        sequence: u8,
    ) -> Self {
        Self {
            profile_id,
            cluster_id,
            source_endpoint,
            destination_endpoint,
            options: options.into(),
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

    #[must_use]
    pub fn options(&self) -> Options {
        let mut options = Options::new();

        if let Ok(Option::None) = Option::try_from(self.options) {
            return options;
        }

        if Option::Retry.into() & self.options {
            options.push(Option::Retry).expect("buffer overflow");
        }

        if Option::EnableRouteDiscovery.into() & self.options {
            options
                .push(Option::EnableRouteDiscovery)
                .expect("buffer overflow");
        }

        if Option::ForceRouteDiscovery.into() & self.options {
            options
                .push(Option::ForceRouteDiscovery)
                .expect("buffer overflow");
        }

        if Option::SourceEui64.into() & self.options {
            options.push(Option::SourceEui64).expect("buffer overflow");
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
