use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::Status;
use crate::error::Resolve;
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x0010;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
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
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Result = ();

    fn resolve(self) -> Result<Self::Result, Error> {
        Status::try_from(self.status).resolve()
    }
}
