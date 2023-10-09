mod policy;

use crate::config;
use policy::Policy;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Stack {
    config: HashMap<config::Id, u8>,
    policy: HashMap<Policy, crate::decision::Id>,
}

impl Stack {
    pub fn new(config: HashMap<config::Id, u8>, policy: HashMap<Policy, Decision>) -> Self {
        Self { config, policy }
    }

    pub fn config(&self) -> &HashMap<config::Id, u8> {
        &self.config
    }

    pub fn policy(&self) -> &HashMap<Policy, Decision> {
        &self.policy
    }
}
