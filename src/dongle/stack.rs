use crate::config;
use crate::decision;
use crate::policy;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Stack {
    config: HashMap<config::Id, u8>,
    policy: HashMap<policy::Id, decision::Id>,
}

impl Stack {
    pub fn new(config: HashMap<config::Id, u8>, policy: HashMap<policy::Id, decision::Id>) -> Self {
        Self { config, policy }
    }

    pub fn config(&self) -> &HashMap<config::Id, u8> {
        &self.config
    }

    pub fn policy(&self) -> &HashMap<policy::Id, decision::Id> {
        &self.policy
    }
}
