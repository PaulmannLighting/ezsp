mod config;
mod decision;
mod policy;

use config::Config;
use decision::Decision;
use policy::Policy;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Stack {
    config: HashMap<Config, u8>,
    policy: HashMap<Policy, Decision>,
}

impl Stack {
    pub fn new(config: HashMap<Config, u8>, policy: HashMap<Policy, Decision>) -> Self {
        Self { config, policy }
    }

    pub fn config(&self) -> &HashMap<Config, u8> {
        &self.config
    }

    pub fn policy(&self) -> &HashMap<Policy, Decision> {
        &self.policy
    }
}
