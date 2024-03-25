use le_stream::derive::{FromLeBytes, ToLeBytes};
use le_stream::{FromLeBytes, ToLeBytes};
use std::fmt::Debug;

pub const ID: u16 = 0x0000;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    desired_protocol_version: u8,
}

impl Command {
    #[must_use]
    pub const fn new(desired_protocol_version: u8) -> Self {
        Self {
            desired_protocol_version,
        }
    }

    #[must_use]
    pub const fn desired_protocol_version(&self) -> u8 {
        self.desired_protocol_version
    }
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response<V>
where
    V: Clone + Copy + Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
{
    protocol_version: u8,
    stack_type: u8,
    stack_version: V,
}

impl<V> Response<V>
where
    V: Clone + Copy + Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
{
    #[must_use]
    pub const fn new(protocol_version: u8, stack_type: u8, stack_version: V) -> Self {
        Self {
            protocol_version,
            stack_type,
            stack_version,
        }
    }

    #[must_use]
    pub const fn protocol_version(&self) -> u8 {
        self.protocol_version
    }

    #[must_use]
    pub const fn stack_type(&self) -> u8 {
        self.stack_type
    }

    #[must_use]
    pub const fn stack_version(&self) -> V {
        self.stack_version
    }
}
