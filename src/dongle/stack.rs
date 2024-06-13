use std::collections::HashMap;

use crate::ezsp::{config, decision, policy};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Stack {
    config: HashMap<config::Id, u8>,
    policy: HashMap<policy::Id, decision::Id>,
}

impl Stack {
    #[must_use]
    pub const fn new(
        config: HashMap<config::Id, u8>,
        policy: HashMap<policy::Id, decision::Id>,
    ) -> Self {
        Self { config, policy }
    }

    #[must_use]
    pub const fn config(&self) -> &HashMap<config::Id, u8> {
        &self.config
    }

    #[must_use]
    pub const fn policy(&self) -> &HashMap<policy::Id, decision::Id> {
        &self.policy
    }
}
