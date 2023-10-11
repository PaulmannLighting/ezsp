use crate::frame::header::{Header, LegacyHeader};

mod configuration;
mod header;

pub trait Frame<const ID: u8> {
    fn header(&self) -> &Header;
}

pub trait LegacyFrame<const ID: u8> {
    fn header(&self) -> &LegacyHeader;
}
