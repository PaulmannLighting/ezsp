use le_stream::derive::FromLeStream;

const ID: u16 = 0x0007;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response;
