use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0010;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    on: bool,
    concentrator_type: u16,
    min_time: u16,
    max_time: u16,
    route_error_threshold: u8,
    delivery_failure_threshold: u8,
    max_hops: u8,
}

impl Command {
    #[must_use]
    pub const fn new(
        on: bool,
        concentrator_type: u16,
        min_time: u16,
        max_time: u16,
        route_error_threshold: u8,
        delivery_failure_threshold: u8,
        max_hops: u8,
    ) -> Self {
        Self {
            on,
            concentrator_type,
            min_time,
            max_time,
            route_error_threshold,
            delivery_failure_threshold,
            max_hops,
        }
    }

    #[must_use]
    pub const fn on(&self) -> bool {
        self.on
    }

    #[must_use]
    pub const fn concentrator_type(&self) -> u16 {
        self.concentrator_type
    }

    #[must_use]
    pub const fn min_time(&self) -> u16 {
        self.min_time
    }

    #[must_use]
    pub const fn max_time(&self) -> u16 {
        self.max_time
    }

    #[must_use]
    pub const fn route_error_threshold(&self) -> u8 {
        self.route_error_threshold
    }

    #[must_use]
    pub const fn delivery_failure_threshold(&self) -> u8 {
        self.delivery_failure_threshold
    }

    #[must_use]
    pub const fn max_hops(&self) -> u8 {
        self.max_hops
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
