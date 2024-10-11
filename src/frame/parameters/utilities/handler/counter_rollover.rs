use le_stream::derive::FromLeStream;

use crate::ember::counter::Type;
use crate::frame;
use crate::frame::Parameter;

const ID: u16 = 0x00F2;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    typ: u8,
}

impl Handler {
    pub fn typ(&self) -> Result<Type, u8> {
        Type::try_from(self.typ)
    }
}

impl Parameter<frame::Extended<frame::Response>> for Handler {
    const ID: u16 = ID;
}
