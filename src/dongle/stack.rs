use crate::ezsp::{config, decision, policy};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Stack {
    config: HashMap<config::Id, u8>,
    policy: HashMap<policy::Id, decision::Id>,
}

impl Stack {
    pub const fn new(
        config: HashMap<config::Id, u8>,
        policy: HashMap<policy::Id, decision::Id>,
    ) -> Self {
        Self { config, policy }
    }

    pub const fn config(&self) -> &HashMap<config::Id, u8> {
        &self.config
    }

    pub const fn policy(&self) -> &HashMap<policy::Id, decision::Id> {
        &self.policy
    }
}
