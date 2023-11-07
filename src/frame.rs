pub mod header;
pub mod legacy_parameters;
pub mod parameters;

pub use header::{Header, LegacyHeader};

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    AddEndpoint(Header, parameters::configuration::add_endpoint::Command),
    //todo!()
}

#[derive(Debug, Eq, PartialEq)]
pub enum Response {
    AddEndpoint(Header, parameters::configuration::add_endpoint::Response),
    //todo!()
}

#[derive(Debug, Eq, PartialEq)]
pub enum Callback {
    //todo!()
}
