pub mod header;
pub mod parameters;

pub use header::{Header, LegacyHeader};
use le_stream::derive::FromLeBytes;
use le_stream::FromLeBytes;

#[derive(Debug, FromLeBytes)]
pub struct ResponseFrame<P>
where
    P: FromLeBytes,
{
    header: Header,
    parameters: P,
}

impl<P> ResponseFrame<P>
where
    P: FromLeBytes,
{
    pub fn parameters(self) -> P {
        self.parameters
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    //todo!()
}

#[derive(Debug, Eq, PartialEq)]
pub enum Response {
    //todo!()
}

#[derive(Debug, Eq, PartialEq)]
pub enum Callback {
    //todo!()
}
