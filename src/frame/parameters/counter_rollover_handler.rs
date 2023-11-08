use crate::types::EmberCounterType;
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x00F2;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    typ: EmberCounterType,
}

impl Response {
    #[must_use]
    pub const fn new(typ: EmberCounterType) -> Self {
        Self { typ }
    }

    #[must_use]
    pub const fn typ(&self) -> EmberCounterType {
        self.typ
    }
}
