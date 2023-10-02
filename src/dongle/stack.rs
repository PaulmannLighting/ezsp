mod config;
mod policy;

use config::Config;
use policy::Policy;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Stack {
    config: HashMap<Config, u8>,
    policy: HashMap<Policy, DecisionId>,
}
